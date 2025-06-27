import { Stat, Element, DamageType, BaseScaling, Amplifier } from '../src/types';
import { StatTable, StatFactory, Rotation } from '../pkg/aminus_js';

async function basicTest() {
    console.log("Testing basic WASM module loading...");
    
    try {
        // Test basic StatTable creation
        const stats = new StatTable();
        stats.add(Stat.FlatATK, 100);
        console.log("StatTable created successfully");
        
        // Test basic Rotation creation
        const rotation = new Rotation();
        rotation.addDamageOperation("test", Element.Pyro, DamageType.Normal, BaseScaling.ATK, Amplifier.None, 1, 1.0);
        console.log("Rotation created successfully");
        
        // Test evaluation
        const result = rotation.evaluate(stats);
        console.log(`Evaluation result: ${result}`);
        
        console.log("Basic test passed!");
        
    } catch (error) {
        console.error("Error in basic test:", error);
    }
}

basicTest().catch(console.error); 