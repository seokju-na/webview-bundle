# @wvb/deno

Deno bindings and desktop integration for WebView Bundle.

Desktop window bindings require Deno 2.9.4 or later. Deno 2.9.4 fixes binding
callbacks being lost when `Deno.serve()` initializes the desktop runtime
([denoland/deno#36065](https://github.com/denoland/deno/pull/36065)).

```ts
// Core bindings
import { loadLib, Source } from '@wvb/deno';

// Desktop integration
import { webviewBundle } from '@wvb/deno/desktop';
```

The native-library installer is available at `@wvb/deno/install`.

Run `deno task check` or `deno task test` from `packages/deno`. Tests require the
native library built with `cargo build -p wvb-deno --release` from the repository
root.
