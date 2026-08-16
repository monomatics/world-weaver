import { type Entity, type World } from "./dashboardUniversals.svelte";
import { invoke } from "@tauri-apps/api/core";

class WorldState {
  savedWorldId = $state<string | null>(null);
  savedWorldName = $state<string | null>(null);
  savedWorld = $state<World | null>(null);

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
  savedEntityCategoryName = $state<string | null>(null);

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
  savedEntity = $state<Entity | null>(null);
  isContentDirty = $state(false);
  isSavingContent = $state(false);

  setEntityState(id: string, name: string) {
    this.savedEntityId = id;
    this.savedEntityName = name;
  }

  clearEntityState() {
    this.savedEntityId = null;
    this.savedEntityName = null;
  }
}

export async function fetchEntity(entity_id: string) {
  if (!entity_id) return;
  try {
    const fetchedEntity: Entity = await invoke("get_entity", {
      worldId: worldState.savedWorldId,
      category: entityCategoryState.savedEntityCategoryName,
      entityId: entity_id,
    });
    entityState.savedEntity = fetchedEntity;
  } catch (e) {
    console.error("An error occurred while fetching the entity:", e);
  }
}

export const worldState = new WorldState();
export const entityCategoryState = new EntityCategoryState();
export const entityState = new EntityState();
