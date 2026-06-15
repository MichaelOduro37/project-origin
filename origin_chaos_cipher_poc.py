import numpy as np

def lorenz_attractor(x, y, z, s=10.0, r=28.0, b=2.667):
    x_dot = s * (y - x)
    y_dot = r * x - y - x * z
    z_dot = x * y - b * z
    return x_dot, y_dot, z_dot

def generate_chaotic_key_stream(seed_x, seed_y, seed_z, length, dt=0.01):
    xs = np.empty(length)
    ys = np.empty(length)
    zs = np.empty(length)
    
    xs[0], ys[0], zs[0] = (seed_x, seed_y, seed_z)
    
    for i in range(length - 1):
        x_dot, y_dot, z_dot = lorenz_attractor(xs[i], ys[i], zs[i])
        xs[i + 1] = xs[i] + (x_dot * dt)
        ys[i + 1] = ys[i] + (y_dot * dt)
        zs[i + 1] = zs[i] + (z_dot * dt)
        
    # Extract structural entropy: map continuous chaotic coordinates into a cryptographic byte stream
    # Multiplied to pull the chaotic fractional fluctuations into integer range
    key_stream = np.mod(np.abs(xs * 1000000), 256).astype(np.uint8)
    return key_stream

if __name__ == "__main__":
    # Alice and Bob sync a hyper-specific float tuple (initial condition) via key exchange
    # Any infinitesimal divergence in these numbers produces a completely different stream.
    shared_seed = (0.1000001, 2.050000, 1.050000)
    
    print("Initializing Origin-Cipher Chaotic Attractor...")
    alice_key = generate_chaotic_key_stream(*shared_seed, length=50000)
    bob_key = generate_chaotic_key_stream(*shared_seed, length=50000)
    
    # 1. Deterministic Proof
    assert np.array_equal(alice_key, bob_key), "Failure: Keys diverged!"
    print("[SUCCESS] Deterministic Proof Passed: Alice and Bob generated identical 50,000-byte keys locally.")
    
    # 2. Butterfly Effect Proof (Extreme Sensitivity)
    eve_seed = (0.10000010000001, 2.050000, 1.050000) # Eve's seed is off by a microscopic fraction
    eve_key = generate_chaotic_key_stream(*eve_seed, length=50000)
    
    match_rate = np.mean(alice_key == eve_key)
    assert match_rate < 0.05, f"Failure: Eve's key was too similar ({match_rate * 100}%)"
    print(f"[SUCCESS] Butterfly Effect Proof Passed: A microscopic seed deviation resulted in totally divergent keys (Match rate: {match_rate*100:.2f}%).")
    
    print(f"\nSample Byte Stream (Hex): {''.join(f'{b:02x}' for b in alice_key[:16])}")
