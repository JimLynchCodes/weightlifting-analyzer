# Snatch Core

a reusable Rust library that handles:

- Pose processing

- Angle math

- Fault detection

- Scoring

- Future ML logic



## Concept of "Phases"

🧠 What Are “Phases”?

For a side-view snatch, we care about:

1. Setup

2. First Pull

3. Transition (knee re-bend)

4. Second Pull (explosive extension)

5. Turnover

6. Catch

7. Recovery (optional)

Without phase detection:

- We can’t detect early arm bend (relative to extension).

- We can’t detect incomplete extension properly.

- We can’t detect late pull timing.

- We can’t measure hip extension timing correctly.

Phases give context.

<br/>

🧠 How Do We Detect Phases?

Using bar velocity + joint angles.

We use:

- Bar vertical velocity

- Hip angle

- Knee angle


Key signals:

Event	             |     Signal
Bar leaves ground    |     velocity > small threshold
Knee re-bend	     |     knee angle decreases after initial extension
Full extension	     |     hip angle peak
Catch	             |     bar descending + deep hip angle


We’re going to approximate these with simple heuristics.