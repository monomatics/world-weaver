export interface World {
  id: string;
  name: string;
  created_at: number;
  modified_at: number;
  description: string;
}

export interface Entity {
  id: string;
  name: string;
  category: string;
  thumbnail_path: string;
  metadata: Record<string, string>;
  content: string;
  tags: string[];
  world_id: string;
  created_at: number;
  modified_at: number;
}
