# EDED
Embedded Distributed Encrypted Database.

*Research projects to support ESSE.*

WIP

#### Distributed design features
- Adapt to personal distributed usecase.
- Use simple (lightweight) async Raft algorithm. (personal devices)
- Optional default leader. (if no default, use Raft). (Cloud or core device)
- Optional just follower. (only vote, and sync, not campaign). (Small embedded device)
- Not only distributed log(database), but also distributed files.
- Database is fully synchronous.
- Optional files is fixed and threshold storage, distributed access.
- Support disk & memory storage

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
