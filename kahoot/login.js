import init, { run_index } from './pkg/yewchat.js';
async function main() {
   await init('/static/pkg/yewchat_bg.wasm');
   run_index();
}
main()
