import { resolve } from 'node:path'
import { defineConfig } from 'vite'
import { builtinModules } from "node:module";

const externalPackages = ['basic-ftp']
const external = [...externalPackages, ...builtinModules, ...builtinModules.map((m) => `node:${m}`)];

export default defineConfig({
    build: {
        // See https://vitejs.dev/guide/build.html#library-mode
        lib: {
            entry: resolve(__dirname, 'lib/client.ts'),
            name: '@nimbuspulse/client'
        },

        rollupOptions: {
            output: {
                format: 'esm',
                inlineDynamicImports: true,
            },
            preserveSymlinks: true,
            external
        },

        // Leaving this unminified so you can see what exactly gets included in
        // the bundles
        minify: false,
    },
})
