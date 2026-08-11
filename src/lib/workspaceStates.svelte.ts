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

class EntityState {
    savedEntityCategoryName = $state<string | null>(null)

    setEntityState(name: string) {
        this.savedEntityCategoryName = name;
    }

    clearEntityState() {    
        this.savedEntityCategoryName = null;
    }
}

    export const worldState = new WorldState
    export const entityState = new EntityState