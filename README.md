# Weightlifting Analyzer(s)

Yerrrrp


## What's Here?

- snatch core: contains a Rust library for:

Pose processing

Angle math

Fault detection

Scoring

Future ML logic


<br/>

## GOAL 1 - Image Detection & Drawing Overlays 

The purpose of this application is to help people improve their technique in the olympic weightlifting move, the _snatch._

From a user point of view:

upload a view (from side view) of a person doing a snatch.

Get a new video that:

- Shows angle between knee, hip, and ankle

- Shows back angle

- Shows bar path

** Also plots a graph of each of these three values over time (similar to MACD or RSI in trading charts)


## Goal 2 - Fault Detection & Coaching Cues

Detects:

Early arm bend

Forward drift

Hip extension timing


Eg other common faults:

⚡ Early Arm Bend

Detection: elbow angle decreases too soon during second pull

Keypoints: shoulder, elbow, wrist

Tip: “Keep arms straight until full hip extension is reached”

⚡ Knee Not Fully Extended

Detection: knee angle < 160° at peak hip extension

Keypoints: hip, knee, ankle

Tip: “Focus on explosive extension of hips and knees”

⚡ Torso Lean Forward

Detection: torso angle too far from vertical (> 25°) at catch

Keypoints: shoulder, hip

Tip: “Keep chest up during pull; maintain upright torso”

⚡ Forward Bar Drift

Detection: bar x-coordinate moves forward relative to feet midline > threshold

Keypoints: wrist midpoint or bar

Tip: “Bar should stay close to body; avoid looping forward”

⚡ Shallow Catch

Detection: hip angle at catch > 100° (not deep enough)

Keypoints: hip, knee, ankle

Tip: “Catch deeper in squat for stability”

⚡ Jump Forward/Back

Detection: large horizontal displacement of bar or foot from start to catch

Keypoints: bar, feet

Tip: “Land close to starting position; control jump”



Defining key metrics from sideview keypoints

1️⃣ Define the Key Metrics From Side-View Keypoints

For a side-view snatch, the main joint/keypoint-based metrics are:

Metric	Keypoints Needed	Why It Matters
Knee angle	hip, knee, ankle	Detect first pull extension & knee rebend
Hip angle	shoulder, hip, knee	Shows hip drive and extension timing
Torso angle	shoulder, hip	Upright torso vs leaning forward
Bar path	wrist midpoint or bar	Horizontal drift, loop, or forward jump
Catch depth	hip, knee, ankle	Indicates squat depth
Vertical bar velocity	wrist or bar	Peak power & pull timing


## Usage

📦 lib.rs (Public API Layer)

This is what your mobile app calls.
```
pub mod models;
pub mod analysis;

pub use analysis::analyzer::Analyzer;
```

Now the outside world just does:
```
use snatch_analyzer::Analyzer;

let result = Analyzer::analyze_frames(&frames);
```