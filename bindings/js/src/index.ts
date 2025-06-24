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

// For convenience, export common functions that will be available after loading
export async function createStatTable() {
  const wasm = await loadWasm();
  return new wasm.StatTable();
}

export async function createArtifactPiece(rarity: number, level: number, statType: number) {
  const wasm = await loadWasm();
  return new wasm.ArtifactPiece(rarity, level, statType);
}

export async function createArtifactBuilder(
  flower?: any,
  feather?: any,
  sands?: any,
  goblet?: any,
  circlet?: any
) {
  const wasm = await loadWasm();
  return new wasm.ArtifactBuilder(flower, feather, sands, goblet, circlet);
}

// Factory functions using the new StatFactory struct
export async function createStatFactory() {
  const wasm = await loadWasm();
  return new wasm.StatFactory();
}

export async function fetchCharacterBaseStats(name: string) {
  const wasm = await loadWasm();
  return await wasm.fetchCharacterBaseStats(name);
}

export async function fetchWeaponStats(name: string) {
  const wasm = await loadWasm();
  return await wasm.fetchWeaponStats(name);
} 