import pandas as pd
import matplotlib.pyplot as plt
import scienceplots

plt.style.use(['science', 'ieee'])

sieve = pd.read_csv('pitimes.csv', index_col=0)

sieve.plot(xlabel='Max', ylabel="Time (s)", title="Extended Sieve")

plt.savefig('sieve.pdf')
