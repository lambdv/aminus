// Re-export types
export * from './types.js';

// This will be populated after building the WASM module
// For now, we'll export a lazy-loaded module
let wasmModule: any = null;

export async function loadWasm() {
  if (!wasmModule) {
    try {
      // Dynamic import to handle the fact that the WASM module may not exist yet
      const module = await import('../pkg/aminus_js.js');
      wasmModule = module;
      return module;
    } catch (error) {
      console.warn('WASM module not found. Run `npm run build` first.');
      throw error;
    }
  }
  return wasmModule;
}

// Export a function to get the WASM module
export async function getWasm() {
  return await loadWasm();
} 