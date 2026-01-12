# tabula.rs
An experimental dataframe library built in Rust for personal use.

## Modules
`core/` - Data model and invariants. **NOTE:** `core` should not depend on any other module.
`io/` - Serialisation / deserialisation.
`expr/`- Expression and column operations e.g. `col("a") + 1` & `when / then / otherwise`.
`ops/` - Operations on series or dataframe e.g. `groupby / join / sort / filter / aggregate`.
