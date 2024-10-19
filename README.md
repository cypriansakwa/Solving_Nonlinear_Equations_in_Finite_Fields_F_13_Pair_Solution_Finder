## Overview
- This Rust program solves the equation $x^2+y^2=1+7\boldsymbol{\cdot}x^2\boldsymbol{\cdot}y^2$ for all pairs $(x,y)$ in the finite field $\mathbb{F}_{13}$ where $\mathbb{F}\_{13}$ represents integers modulo 13.
## How it works
- The program iterates over all possible pairs $(x,y)$ from 
$\mathbb{F}_{13}\times\mathbb{F}\_{13}$, calculates both sides of the equation modulo 13, and prints all pairs that satisfy the equation.
