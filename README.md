## Body Graph

Body Graph is a small 2D point-and-click graph toy built with Bevy.
It is primarily a **learning project** to explore Bevy’s ECS, input handling, picking, and simple UI for a 2D game-like experience.

### What it does

- **Interactive graph canvas**: click on the canvas to create vertices(/nodes).
- **Move vertices**: drag vertices with the **LMB** to move them.
- **Create directed edges**:
  - **Right-drag** from a vertex and release on empty space to create a new vertex and connect to it.
  - **Right-drag** from a vertex and release on another vertex to connect them with an edge.
  - **Right-drag** from a vertex and release on an existing edge to insert a new vertex on that edge and connect through it.
- **Edit graph structure**:
  - **Ctrl + left click vertex**: delete the vertex (and connected edges).
  - **Ctrl + click edge**: delete the edge.
  - **Click edge**: insert a vertex on the edge at the click point (the original edge is split into two).
- **Rename vertices**:
  - **Double left click** a vertex to open a small egui text field near it.
  - Type the new label and press **Enter** to confirm, or **Esc** to cancel (Probably clicking outside should cancel too).
- **Cursor feedback**:
  - Default pointer on empty space.
  - Grab-like cursor over vertices.
  - Add-like cursor over edges
  - crosshair-like cursor over edges/vertices ctrl is held.

### Initial goals

This project is intentionally small and focused.
It exists to practice:

- Structuring a Bevy app into plugins, components, bundles, systems, events, and resources.
- Using pointer events and simple “picking” to drive in-world interactions.
- Combining Bevy rendering with an immediate-mode UI (`bevy_egui`).
- Implementing basic editor-like behaviors (create/move/delete/rename nodes, insert on edges).

### Possible improvements

- Weights on edges
- Panning the canvas/graph within canvas/camera/etc...
- Right click context menu with whatever options
- A general menu to save/load graphs, and maybe visualize algorithms (probably won't do).
- Think more about how this could be an actual general-purpose plugin at least for myself.
