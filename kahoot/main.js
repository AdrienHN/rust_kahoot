import init, { run_app } from './pkg/yewchat.js';
async function main() {
   await init('/static/pkg/yewchat_bg.wasm');
   run_app();
}
main()
