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

class EntityCategoryState {
    savedEntityCategoryName = $state<string | null>(null)

    setEntityCategoryState(name: string) {
        this.savedEntityCategoryName = name;
    }

    clearEntityCategoryState() {    
        this.savedEntityCategoryName = null;
    }
}

class EntityState {
    savedEntityId = $state<string | null>(null);
    savedEntityName = $state<string | null>(null);

    setEntityState(id: string, name: string) {
        this.savedEntityId = id;
        this.savedEntityName = name;
    }

    clearEntityState() {
        this.savedEntityId = null;
        this.savedEntityName = null;
    }
}

    export const worldState = new WorldState
    export const entityCategoryState = new EntityCategoryState
    export const entityState = new EntityState