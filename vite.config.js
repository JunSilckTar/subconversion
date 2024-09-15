import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
import {join} from 'path';
import {visualizer} from "rollup-plugin-visualizer";


// https://vitejs.dev/config/
export default defineConfig({
    root: 'src',
    plugins: [vue(), visualizer()],
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
