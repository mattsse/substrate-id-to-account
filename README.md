# Print converted Accounts from Parachain or Pallet ids

## Usage

```shell
cargo install --git "https://github.com/mattsse/substrate-id-to-account"
```

Print Account for Parachain id of 200

```shell
 substrate-id-to-account paraid 200
```

`ParaId(200): "5Ec4AhPTL6nWnUnw58QzjJvFd3QATwHA3UJnvSD4GVSQ7Gop"`

Print Account for PalletId (8 byte) id of "12345678"

```shell
 substrate-id-to-account pallet 12345678
```

`PalletId(12345678): "5EYCAe5W4LBgLK1zfXfPz2gWtwbzmFDR2kTENsNyDuV826iS"`

### License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

License: Apache-2.0/MIT