import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
import {join} from 'path';

// https://vitejs.dev/config/
export default defineConfig({
    root: 'src',
    plugins: [vue()],
    server: {
        port: 3000
    },
    build: {
        outDir: '../dist',
    },
    resolve: {
        alias: {
            '@@': join(__dirname, '/src'),
        }
    },
})
