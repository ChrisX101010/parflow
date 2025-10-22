import cmath
import numpy as np

def fft(x):
    N = len(x)
    if N <= 1:
        return x
    even = fft(x[0::2])
    odd = fft(x[1::2])
    T = [cmath.exp(-2j * cmath.pi * k / N) * odd[k] for k in range(N // 2)]
    return [even[k] + T[k] for k in range(N // 2)] + [even[k] - T[k] for k in range(N // 2)]

# Generate a random signal with 8 points
x = np.random.random(8)

# Compute the FFT using custom implementation
fft_custom = fft(x)

# Compute the FFT using NumPy
fft_numpy = np.fft.fft(x)

# Compare results
print("Input:", x)
print("Custom FFT:", fft_custom)
print("NumPy FFT:", fft_numpy)
print("Difference:", np.allclose(fft_custom, fft_numpy))
