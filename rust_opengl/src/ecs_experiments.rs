use std::collections::HashMap;
use std::ops::Add;
use std::rc::Rc;
use std::iter::Map;

type EntityId = u64;
type ComponentId = EntityId;
type EntityType = Vec<ComponentId>;
type EntityIndex<T : Component> = HashMap<EntityId, Record<T>>;
type ComponentArray<T : Component> = Vec<T>;

static ENTITY_ID_COUNTER : EntityId = 0;
static COMPONENT_ID_COUNTER : ComponentId = 1;

trait Entity {
    fn get_id(&self) -> EntityId;
}

trait Component {
    fn get_id(&self) -> ComponentId;
}

/**
 * Archetype references for the archetypes graph
 * Each archetype can reference the same number of archetypes as it's component ids number when removing these ids
 * Each archetype can reference multiple archetypes when adding new component ids (the archetype can either exists or needs to be created)
 */
struct Edge<T : Component> {
    add: Rc<Archetype<T>>,
    remove: Rc<Archetype<T>>
}

/**
 * This class stores arrays of each component belonging to an entity type (components from entity instances that have the same component ids)
 * @entityType array of component ids
 * @components 2d array. Rows are represent the component id
 *             -> for each component id multiple component instances can be stored because multiple entities can have the same component(not necessarily the same instance)
 * @entityIds the ids of all the entities that are of the archetype
 * @edges archetype references for the next archetype based on removing or adding a component
 */
struct Archetype<T : Component> {
    entity_type: EntityType,
    entity_ids: Vec<EntityId>,
    components: Vec<ComponentArray<T>>,
    edges: Vec<Edge<T>>
}

/**
 * @row -> position of the components that belongs to the entity from the archetypes component list
 * @archetype -> the archetype in which the entity belongs
 */
struct Record<T : Component> {
    row: ComponentId,
    archetype: Rc<Archetype<T>>
}


impl<T> Archetype<T> where T : Component {
    fn new(entity_type: EntityType, entity_ids: Vec<EntityId>, components: Vec<ComponentArray<T>>, edges: Vec<Edge<T>>) -> Archetype<T> {
        Archetype {
            entity_type,
            entity_ids,
            components,
            edges,
        }
    }
}

struct RootComponent {}

impl Component for RootComponent {
    fn get_id(&self) -> ComponentId {
       0
    }
}

pub fn add_components(entity: &dyn Entity, components: Vec<Box<dyn Component>>) {

}

pub fn ecs_test() {
    let root_archetype: Archetype<RootComponent> = Archetype::new(Vec::with_capacity(1), Vec::with_capacity(0), Vec::with_capacity(1), Vec::new());
}





