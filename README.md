# Bevy Basic Player

Move with left and right arrows, hold shift to run, jump with space.

If you're learning, make sure to check out the commit history.

The lld setup for *blazingly* fast compile times (0.1s-2s) was extracted from [Bevy's Get Started Guide](https://bevyengine.org/learn/book/getting-started/setup/#create-a-new-bevy-project).

The texture atlas loading strategy and animation logic was extracted from [bevy_asset_loader's atlas from grid example](https://github.com/NiklasEi/bevy_asset_loader/blob/main/bevy_asset_loader/examples/atlas_from_grid.rs)

The rest is just a very basic state machine.

Note that jumping is in a separate branch `extended`.

You should use a proper physics engine as well, like [rapier](https://github.com/dimforge/bevy_rapier). Do not go with my setup for that, physics processing should happen separate from the main process, otherwise you'll get inconsistencies (like the ones you may see in this demo).

## Assets

[zegley's](https://zegley.itch.io/2d-platformermetroidvania-asset-pack)

## SpriteSheet Packer

[codeandweb](https://www.codeandweb.com/free-sprite-sheet-packer)