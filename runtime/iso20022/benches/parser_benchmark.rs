//! Performance benchmarks for ISO 20022 and JSON parsing
//!
//! Run with: cargo bench -p estream-iso20022

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;

// Sample ISO 20022 pacs.008 message (typical credit transfer)
const PACS008_SMALL: &[u8] = br#"<?xml version="1.0"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>MSG-2024-001</MsgId>
      <CreDtTm>2024-01-15T10:30:00Z</CreDtTm>
      <NbOfTxs>1</NbOfTxs>
      <SttlmInf><SttlmMtd>CLRG</SttlmMtd></SttlmInf>
    </GrpHdr>
    <CdtTrfTxInf>
      <PmtId><InstrId>INSTR-001</InstrId><EndToEndId>E2E-001</EndToEndId></PmtId>
      <IntrBkSttlmAmt Ccy="USD">1000.00</IntrBkSttlmAmt>
      <ChrgBr>SHAR</ChrgBr>
      <Dbtr><Nm>John Doe</Nm></Dbtr>
      <DbtrAcct><Id><IBAN>DE89370400440532013000</IBAN></Id></DbtrAcct>
      <DbtrAgt><FinInstnId><BICFI>COBADEFFXXX</BICFI></FinInstnId></DbtrAgt>
      <CdtrAgt><FinInstnId><BICFI>BNPAFRPPXXX</BICFI></FinInstnId></CdtrAgt>
      <Cdtr><Nm>Jane Smith</Nm></Cdtr>
      <CdtrAcct><Id><IBAN>FR7630006000011234567890189</IBAN></Id></CdtrAcct>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#;

// Equivalent JSON representation
const PACS008_JSON_SMALL: &[u8] = br#"{
  "Document": {
    "FIToFICstmrCdtTrf": {
      "GrpHdr": {
        "MsgId": "MSG-2024-001",
        "CreDtTm": "2024-01-15T10:30:00Z",
        "NbOfTxs": 1,
        "SttlmInf": {"SttlmMtd": "CLRG"}
      },
      "CdtTrfTxInf": {
        "PmtId": {"InstrId": "INSTR-001", "EndToEndId": "E2E-001"},
        "IntrBkSttlmAmt": {"value": 1000.00, "Ccy": "USD"},
        "ChrgBr": "SHAR",
        "Dbtr": {"Nm": "John Doe"},
        "DbtrAcct": {"Id": {"IBAN": "DE89370400440532013000"}},
        "DbtrAgt": {"FinInstnId": {"BICFI": "COBADEFFXXX"}},
        "CdtrAgt": {"FinInstnId": {"BICFI": "BNPAFRPPXXX"}},
        "Cdtr": {"Nm": "Jane Smith"},
        "CdtrAcct": {"Id": {"IBAN": "FR7630006000011234567890189"}}
      }
    }
  }
}"#;

// Generate a large XML message with multiple transactions
fn generate_large_pacs008(num_transactions: usize) -> Vec<u8> {
    let mut xml = String::from(r#"<?xml version="1.0"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>MSG-BULK-001</MsgId>
      <CreDtTm>2024-01-15T10:30:00Z</CreDtTm>
      <NbOfTxs>"#);
    xml.push_str(&num_transactions.to_string());
    xml.push_str(r#"</NbOfTxs>
      <SttlmInf><SttlmMtd>CLRG</SttlmMtd></SttlmInf>
    </GrpHdr>"#);
    
    for i in 0..num_transactions {
        xml.push_str(&format!(r#"
    <CdtTrfTxInf>
      <PmtId><InstrId>INSTR-{:06}</InstrId><EndToEndId>E2E-{:06}</EndToEndId></PmtId>
      <IntrBkSttlmAmt Ccy="USD">{}.00</IntrBkSttlmAmt>
      <ChrgBr>SHAR</ChrgBr>
      <Dbtr><Nm>Debtor {}</Nm></Dbtr>
      <DbtrAcct><Id><IBAN>DE89370400440532013{:03}</IBAN></Id></DbtrAcct>
      <DbtrAgt><FinInstnId><BICFI>COBADEFFXXX</BICFI></FinInstnId></DbtrAgt>
      <CdtrAgt><FinInstnId><BICFI>BNPAFRPPXXX</BICFI></FinInstnId></CdtrAgt>
      <Cdtr><Nm>Creditor {}</Nm></Cdtr>
      <CdtrAcct><Id><IBAN>FR76300060000112345678901{:02}</IBAN></Id></CdtrAcct>
    </CdtTrfTxInf>"#, i, i, 1000 + i, i, i % 1000, i, i % 100));
    }
    
    xml.push_str(r#"
  </FIToFICstmrCdtTrf>
</Document>"#);
    
    xml.into_bytes()
}

// Generate large JSON message
fn generate_large_json(num_transactions: usize) -> Vec<u8> {
    let mut json = String::from(r#"{"Document":{"FIToFICstmrCdtTrf":{"GrpHdr":{"MsgId":"MSG-BULK-001","CreDtTm":"2024-01-15T10:30:00Z","NbOfTxs":"#);
    json.push_str(&num_transactions.to_string());
    json.push_str(r#","SttlmInf":{"SttlmMtd":"CLRG"}},"CdtTrfTxInf":["#);
    
    for i in 0..num_transactions {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&format!(
            r#"{{"PmtId":{{"InstrId":"INSTR-{:06}","EndToEndId":"E2E-{:06}"}},"IntrBkSttlmAmt":{{"value":{},"Ccy":"USD"}},"ChrgBr":"SHAR","Dbtr":{{"Nm":"Debtor {}"}},"DbtrAcct":{{"Id":{{"IBAN":"DE89370400440532013{:03}"}}}},"DbtrAgt":{{"FinInstnId":{{"BICFI":"COBADEFFXXX"}}}},"CdtrAgt":{{"FinInstnId":{{"BICFI":"BNPAFRPPXXX"}}}},"Cdtr":{{"Nm":"Creditor {}"}},"CdtrAcct":{{"Id":{{"IBAN":"FR76300060000112345678901{:02}"}}}}}}"#,
            i, i, 1000 + i, i, i % 1000, i, i % 100
        ));
    }
    
    json.push_str(r#"]}}}"#);
    json.into_bytes()
}

/// Simple streaming tokenizer for benchmarking
mod tokenizer {
    /// XML token types
    #[derive(Debug, Clone, Copy)]
    pub enum XmlToken {
        StartElement,
        EndElement,
        Attribute,
        Text,
        Comment,
    }

    /// JSON token types
    #[derive(Debug, Clone, Copy)]
    pub enum JsonToken {
        ObjectStart,
        ObjectEnd,
        ArrayStart,
        ArrayEnd,
        Key,
        String,
        Number,
        Boolean,
        Null,
    }

    /// Simple XML tokenizer (baseline)
    pub fn tokenize_xml(data: &[u8]) -> usize {
        let mut count = 0;
        let mut in_tag = false;
        let mut in_string = false;
        
        for &byte in data {
            match byte {
                b'<' if !in_string => {
                    in_tag = true;
                    count += 1;
                }
                b'>' if !in_string => {
                    in_tag = false;
                    count += 1;
                }
                b'"' => {
                    in_string = !in_string;
                }
                _ => {}
            }
        }
        count
    }

    /// Simple JSON tokenizer (baseline)
    pub fn tokenize_json(data: &[u8]) -> usize {
        let mut count = 0;
        let mut in_string = false;
        let mut escape = false;
        
        for &byte in data {
            if escape {
                escape = false;
                continue;
            }
            match byte {
                b'\\' if in_string => {
                    escape = true;
                }
                b'"' => {
                    in_string = !in_string;
                    count += 1;
                }
                b'{' | b'}' | b'[' | b']' | b':' | b',' if !in_string => {
                    count += 1;
                }
                _ => {}
            }
        }
        count
    }

    /// SIMD-accelerated XML tokenizer (parallel character classification)
    #[cfg(target_arch = "x86_64")]
    pub fn tokenize_xml_simd(data: &[u8]) -> usize {
        use std::arch::x86_64::*;
        
        let mut count = 0;
        let chunks = data.chunks_exact(16);
        let remainder = chunks.remainder();
        
        unsafe {
            let lt = _mm_set1_epi8(b'<' as i8);
            let gt = _mm_set1_epi8(b'>' as i8);
            
            for chunk in chunks {
                let v = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);
                let lt_mask = _mm_movemask_epi8(_mm_cmpeq_epi8(v, lt));
                let gt_mask = _mm_movemask_epi8(_mm_cmpeq_epi8(v, gt));
                count += (lt_mask.count_ones() + gt_mask.count_ones()) as usize;
            }
        }
        
        // Handle remainder
        count += tokenize_xml(remainder);
        count
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn tokenize_xml_simd(data: &[u8]) -> usize {
        tokenize_xml(data)
    }

    /// SIMD-accelerated JSON tokenizer
    #[cfg(target_arch = "x86_64")]
    pub fn tokenize_json_simd(data: &[u8]) -> usize {
        use std::arch::x86_64::*;
        
        let mut count = 0;
        let chunks = data.chunks_exact(16);
        let remainder = chunks.remainder();
        
        unsafe {
            let lbrace = _mm_set1_epi8(b'{' as i8);
            let rbrace = _mm_set1_epi8(b'}' as i8);
            let lbracket = _mm_set1_epi8(b'[' as i8);
            let rbracket = _mm_set1_epi8(b']' as i8);
            let quote = _mm_set1_epi8(b'"' as i8);
            
            for chunk in chunks {
                let v = _mm_loadu_si128(chunk.as_ptr() as *const __m128i);
                let m1 = _mm_movemask_epi8(_mm_cmpeq_epi8(v, lbrace));
                let m2 = _mm_movemask_epi8(_mm_cmpeq_epi8(v, rbrace));
                let m3 = _mm_movemask_epi8(_mm_cmpeq_epi8(v, lbracket));
                let m4 = _mm_movemask_epi8(_mm_cmpeq_epi8(v, rbracket));
                let m5 = _mm_movemask_epi8(_mm_cmpeq_epi8(v, quote));
                count += (m1.count_ones() + m2.count_ones() + m3.count_ones() + 
                         m4.count_ones() + m5.count_ones()) as usize;
            }
        }
        
        count += tokenize_json(remainder);
        count
    }

    #[cfg(not(target_arch = "x86_64"))]
    pub fn tokenize_json_simd(data: &[u8]) -> usize {
        tokenize_json(data)
    }
}

/// FNV-1a hash (matches FPGA tree_walker_fsm.v)
fn fnv1a_hash(path: &[u8]) -> u32 {
    const FNV_OFFSET_BASIS: u32 = 0x811c9dc5;
    const FNV_PRIME: u32 = 0x01000193;
    
    let mut hash = FNV_OFFSET_BASIS;
    for &byte in path {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Path hashing benchmark
fn bench_path_hashing(c: &mut Criterion) {
    let paths = [
        b"/Document/FIToFICstmrCdtTrf/GrpHdr/MsgId".as_slice(),
        b"/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/PmtId/InstrId".as_slice(),
        b"/Document/FIToFICstmrCdtTrf/CdtTrfTxInf/IntrBkSttlmAmt".as_slice(),
    ];
    
    c.bench_function("fnv1a_path_hash", |b| {
        b.iter(|| {
            for path in &paths {
                black_box(fnv1a_hash(black_box(path)));
            }
        })
    });
}

/// XML tokenization benchmark
fn bench_xml_tokenize(c: &mut Criterion) {
    let mut group = c.benchmark_group("xml_tokenize");
    group.measurement_time(Duration::from_secs(5));
    
    // Small message
    group.throughput(Throughput::Bytes(PACS008_SMALL.len() as u64));
    group.bench_with_input(BenchmarkId::new("scalar", "small"), PACS008_SMALL, |b, data| {
        b.iter(|| tokenizer::tokenize_xml(black_box(data)))
    });
    
    group.bench_with_input(BenchmarkId::new("simd", "small"), PACS008_SMALL, |b, data| {
        b.iter(|| tokenizer::tokenize_xml_simd(black_box(data)))
    });
    
    // Medium message (100 transactions)
    let medium = generate_large_pacs008(100);
    group.throughput(Throughput::Bytes(medium.len() as u64));
    group.bench_with_input(BenchmarkId::new("scalar", "medium"), &medium, |b, data| {
        b.iter(|| tokenizer::tokenize_xml(black_box(data)))
    });
    
    group.bench_with_input(BenchmarkId::new("simd", "medium"), &medium, |b, data| {
        b.iter(|| tokenizer::tokenize_xml_simd(black_box(data)))
    });
    
    // Large message (1000 transactions)
    let large = generate_large_pacs008(1000);
    group.throughput(Throughput::Bytes(large.len() as u64));
    group.bench_with_input(BenchmarkId::new("scalar", "large"), &large, |b, data| {
        b.iter(|| tokenizer::tokenize_xml(black_box(data)))
    });
    
    group.bench_with_input(BenchmarkId::new("simd", "large"), &large, |b, data| {
        b.iter(|| tokenizer::tokenize_xml_simd(black_box(data)))
    });
    
    group.finish();
}

/// JSON tokenization benchmark
fn bench_json_tokenize(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_tokenize");
    group.measurement_time(Duration::from_secs(5));
    
    // Small message
    group.throughput(Throughput::Bytes(PACS008_JSON_SMALL.len() as u64));
    group.bench_with_input(BenchmarkId::new("scalar", "small"), PACS008_JSON_SMALL, |b, data| {
        b.iter(|| tokenizer::tokenize_json(black_box(data)))
    });
    
    group.bench_with_input(BenchmarkId::new("simd", "small"), PACS008_JSON_SMALL, |b, data| {
        b.iter(|| tokenizer::tokenize_json_simd(black_box(data)))
    });
    
    // Medium message (100 transactions)
    let medium = generate_large_json(100);
    group.throughput(Throughput::Bytes(medium.len() as u64));
    group.bench_with_input(BenchmarkId::new("scalar", "medium"), &medium, |b, data| {
        b.iter(|| tokenizer::tokenize_json(black_box(data)))
    });
    
    group.bench_with_input(BenchmarkId::new("simd", "medium"), &medium, |b, data| {
        b.iter(|| tokenizer::tokenize_json_simd(black_box(data)))
    });
    
    // Large message (1000 transactions)
    let large = generate_large_json(1000);
    group.throughput(Throughput::Bytes(large.len() as u64));
    group.bench_with_input(BenchmarkId::new("scalar", "large"), &large, |b, data| {
        b.iter(|| tokenizer::tokenize_json(black_box(data)))
    });
    
    group.bench_with_input(BenchmarkId::new("simd", "large"), &large, |b, data| {
        b.iter(|| tokenizer::tokenize_json_simd(black_box(data)))
    });
    
    group.finish();
}

/// XML vs JSON comparison
fn bench_xml_vs_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("xml_vs_json");
    
    // Same logical content, different formats
    let xml = generate_large_pacs008(100);
    let json = generate_large_json(100);
    
    println!("XML size: {} bytes", xml.len());
    println!("JSON size: {} bytes", json.len());
    
    group.throughput(Throughput::Elements(100)); // 100 transactions
    
    group.bench_function("xml", |b| {
        b.iter(|| tokenizer::tokenize_xml_simd(black_box(&xml)))
    });
    
    group.bench_function("json", |b| {
        b.iter(|| tokenizer::tokenize_json_simd(black_box(&json)))
    });
    
    group.finish();
}

/// Throughput measurement
fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");
    group.measurement_time(Duration::from_secs(10));
    
    // 10K transactions - simulate high-volume processing
    let xml_10k = generate_large_pacs008(10000);
    let json_10k = generate_large_json(10000);
    
    println!("XML 10K size: {} MB", xml_10k.len() as f64 / 1_000_000.0);
    println!("JSON 10K size: {} MB", json_10k.len() as f64 / 1_000_000.0);
    
    group.throughput(Throughput::Bytes(xml_10k.len() as u64));
    group.bench_function("xml_10k_txns", |b| {
        b.iter(|| tokenizer::tokenize_xml_simd(black_box(&xml_10k)))
    });
    
    group.throughput(Throughput::Bytes(json_10k.len() as u64));
    group.bench_function("json_10k_txns", |b| {
        b.iter(|| tokenizer::tokenize_json_simd(black_box(&json_10k)))
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_path_hashing,
    bench_xml_tokenize,
    bench_json_tokenize,
    bench_xml_vs_json,
    bench_throughput,
);

criterion_main!(benches);
