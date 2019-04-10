This is a script for demo-ing `spor`. It's based on the `follow-history` tool applied to a specific version of `anchor.rs`. 

1. Copy original version of `anchor.rs`
```
cp history/00-anchor.rs anchor.rs
```

2. Initialize spor repository
```
cargo run init
```

3. Create anchor
```
echo "{'meta': 'data'}" | cargo run add anchor.rs 25 86 10
```

Examine the anchor file.

4. Demonstrate `spor` commands
```
cargo run list anchor.rs
cargo run details <anchor id>
cargo run status
```

5. "edit" anchor.rs
```
cp history/01-anchor.rs anchor.rs
```

6. Show status and diff
```
cargo run status
cargo run status <anchor id>
```

7. Update the anchor

Show the anchor in the editor so we can see it change.

```
cargo run update
```

We should see the anchor updated to match the new source.

8. Repeat edit/update cycle through version 03 

At this point, spor gets things a bit wrong. We no longer anchor the entire struct.

9. Replace anchor with new one

```
rm .spor/<anchor file>
echo "{}" | cargo run add 155 101 10 
```

10. Continue "editing"

```
cp history/04-anchor.rs anchor.rs
```