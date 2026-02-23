# Migration Guide: {{component}} v{{old}} -> v{{new}}

> **Status**: {{status}}
> **Sunset Date**: {{sunset_date}}
> **Successor**: {{successor}}

## Breaking Changes

| Change | Before | After | Migration |
|--------|--------|-------|-----------|
| _describe change_ | _old behavior_ | _new behavior_ | _steps to migrate_ |

## Schema Changes

_List any data schema additions, removals, or modifications._

## Circuit Changes

_List any circuit signature changes, new inputs/outputs, removed circuits._

## Step-by-Step Migration

1. Update `estream-component.toml` dependency version
2. Run `estream marketplace upgrade --dry-run` to preview changes
3. Update circuit calls to match new signatures
4. Run `estream test` to verify
5. Deploy with `estream deploy --shadow` for side-by-side validation

## Support

If you need help migrating, reach out via the eStream marketplace support channel.
