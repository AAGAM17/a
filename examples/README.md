# Usage Examples for A Package Manager

This directory contains examples showing how to use A package manager in different project scenarios.

## React App

The `react-app` directory contains an example React application set up with A package manager.

To try it:

```bash
cd react-app
a install
a run dev
```

## Quick Start

### 1. Initialize a new project

```bash
mkdir my-new-project
cd my-new-project
a init
```

### 2. Add dependencies

```bash
# Add a production dependency
a add react

# Add a development dependency
a add --dev jest
```

### 3. Install all dependencies

```bash
a install
```

### 4. Run scripts

First, add scripts to your a.json file:

```json
{
  "scripts": {
    "start": "node index.js",
    "test": "jest"
  }
}
```

Then run them:

```bash
a run start
a run test
```

### 5. Update dependencies

```bash
# Update all
a update

# Update specific package
a update react
```
