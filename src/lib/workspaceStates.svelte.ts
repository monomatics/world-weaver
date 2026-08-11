class WorldState {
    savedWorldId = $state<string | null>(null);
    savedWorldName = $state<string | null>(null);

    setWorldState(id: string, name: string) {
        this.savedWorldId = id;
        this.savedWorldName = name;
    }

    clearWorldState() {
        this.savedWorldId = null;
        this.savedWorldName = null;
    }
}

export const worldState = new WorldState