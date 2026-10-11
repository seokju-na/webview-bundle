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
