🎯 **HYDROGEN ORBITAL SHAPES VISUALIZATION**

## 1s Orbital (Spherically Symmetric)
```
        ●●●●●
      ●●●●●●●●●
    ●●●●●●●●●●●●●
   ●●●●●●●●●●●●●●●
  ●●●●●●●●●●●●●●●●●
 ●●●●●●●●●●●●●●●●●●●
●●●●●●●●●[H]●●●●●●●●●  ← Nucleus at center
 ●●●●●●●●●●●●●●●●●●●
  ●●●●●●●●●●●●●●●●●
   ●●●●●●●●●●●●●●●
    ●●●●●●●●●●●●●
      ●●●●●●●●●
        ●●●●●
```
**Characteristics:**
- Highest probability at nucleus (density = 0.318)
- Spherically symmetric (same in all directions)
- Exponential decay with distance
- Ground state orbital

## 2pz Orbital (Dumbbell Along Z-axis)
```
     ●●●●●●●●●
   ●●●●●●●●●●●●●
  ●●●●●●●●●●●●●●●
 ●●●●●●●●●●●●●●●●●
●●●●●●●●●●●●●●●●●●●
●●●●●●●●●●●●●●●●●●●
●●●●●●●●●●●●●●●●●●●
     
      [NODAL PLANE]    ← Zero probability at z=0
     
●●●●●●●●●●●●●●●●●●●
●●●●●●●●●●●●●●●●●●●
●●●●●●●●●●●●●●●●●●●
 ●●●●●●●●●●●●●●●●●
  ●●●●●●●●●●●●●●●
   ●●●●●●●●●●●●●
     ●●●●●●●●●
```
**Characteristics:**
- Zero probability at origin and XY-plane
- Maximum along Z-axis (density = 0.000229)
- Dumbbell/figure-8 shape
- Nodal plane at z = 0

## 2px Orbital (Dumbbell Along X-axis)
```
                ●●●●●●●●●
              ●●●●●●●●●●●●●
             ●●●●●●●●●●●●●●●
            ●●●●●●●●●●●●●●●●●
           ●●●●●●●●●●●●●●●●●●●
  [NODAL   ●●●●●●●●●●●●●●●●●●●
   PLANE]         [H]         [NODAL PLANE]
           ●●●●●●●●●●●●●●●●●●●
            ●●●●●●●●●●●●●●●●●
             ●●●●●●●●●●●●●●●
              ●●●●●●●●●●●●●
                ●●●●●●●●●
```
**Characteristics:**
- Zero probability at origin and YZ-plane (x=0)
- Maximum along X-axis
- Nodal plane at x = 0

## 3dz² Orbital (Complex D-orbital)
```
        ●●●●●●●●●●●●●
      ●●●●●●●●●●●●●●●●●
     ●●●●●●●●●●●●●●●●●●●
    ●●●●●●●●●●●●●●●●●●●●●
   ●●●●●●●●●●●●●●●●●●●●●●●
  ●●●●●●●●●●●●●●●●●●●●●●●●●

    ●●●     ●●●     ●●●     ← Ring in XY-plane
   ●●●●●   ●●●●●   ●●●●●
  ●●●●●●● ●●●●●●● ●●●●●●●
   ●●●●●   ●●●●●   ●●●●●
    ●●●     ●●●     ●●●

  ●●●●●●●●●●●●●●●●●●●●●●●●●
   ●●●●●●●●●●●●●●●●●●●●●●●
    ●●●●●●●●●●●●●●●●●●●●●
     ●●●●●●●●●●●●●●●●●●●
      ●●●●●●●●●●●●●●●●●
        ●●●●●●●●●●●●●
```
**Characteristics:**
- Zero probability at origin
- Higher probability along Z-axis
- Ring of probability in XY-plane
- Complex nodal surfaces
- Largest orbital (extends furthest from nucleus)

## 📊 **Data From Our Demo:**

| Orbital | Acceptance Rate | Median Distance | Key Features |
|---------|----------------|----------------|--------------|
| 1s      | 65.5%          | 1.409         | Compact, spherical |
| 2pz     | 83.2%          | 4.385         | Extended along Z |
| 2px     | 84.8%          | 4.938         | Extended along X |
| 3dz²    | 90.0%          | 13.328        | Very extended, complex |

## 🎨 **In the Real 3D Visualization:**

When you run `cargo run --release`, you'd see:
- **Interactive 3D point clouds** forming these exact shapes
- **Mouse controls** to rotate and examine from any angle
- **Zoom functionality** to see fine details
- **Real-time rendering** with thousands of points
- **Color-coded density** (more points = higher probability)
- **Smooth orbital boundaries** from Monte Carlo sampling

The app uses the **Metropolis-Hastings algorithm** to sample points according to quantum mechanical probability distributions, creating authentic orbital visualizations that match theoretical predictions!