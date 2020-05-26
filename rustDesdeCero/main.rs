use std::cmp::Ordering;
use std::collections::HashSet;
use core::fmt;
/*-----------------------------------Arista--------------------------------------*/
#[derive(Clone, Debug)]
pub struct Arista {
    pub lazo: (String, String),
    pub distancia: u64,
}
/*------------------------------------Vertice---------------------------------*/
#[derive(Clone, Debug, Eq)]
pub struct Vertice {
    pub id: String,
    pub distancia: (Option<String>, Option<u64>),
}

impl Vertice {
    pub fn new(n: &str) -> Vertice {
        Vertice {
            id: n.to_string(),
            distancia: (None, None),
        }
    }
}

impl Ord for Vertice {
    fn cmp(&self, other: &Vertice) -> Ordering {
        if self.distancia.1.is_none() {
            return Ordering::Greater;
        }
        if other.distancia.1.is_none() {
            return Ordering::Less;
        }
        self.distancia.1.cmp(&other.distancia.1)
    }
}

impl PartialOrd for Vertice {
    fn partial_cmp(&self, other: &Vertice) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vertice {
    fn eq(&self, other: &Vertice) -> bool {
        self.distancia.1 == other.distancia.1
    }
}

/*------------------------------------Grafo-----------------------------------------*/
#[derive(Clone)]
pub struct Grafo {
    pub vertices: Vec<Vertice>,
    pub aristas: Vec<Arista>,
}

impl Grafo {
    pub fn new(dato: Vec<(u64, &str, &str)>) -> Grafo {
        let vs: HashSet<&str> = dato.iter()
            .flat_map(|&(_, a, b)| vec![a, b].into_iter())
            .collect();

        let vers = vs.iter().map(|x| Vertice::new(x)).collect();

        let aris = dato.iter()
            .map(|&(dis, origen, destino)| {
                Arista {
                    lazo: (origen.to_string(), destino.to_string()),
                    distancia: dis,
                }
            })
            .collect();

        Grafo {
            vertices: vers,
            aristas: aris,
        }
    }

    pub fn buscar_id_mut(&mut self, id: &str) -> Option<&mut Vertice> {
        self.vertices.iter_mut().find(|x| x.id == id.to_string())
    }

    pub fn buscar_vecinos(&self, id: &str) -> Vec<(u64, String)> {
        self.aristas
            .iter()
            .filter(|x| x.lazo.0 == id.to_string() || x.lazo.1 == id.to_string())
            .map(|x| if x.lazo.0 == id.to_string() {
                (x.distancia.clone(), x.lazo.1.clone())
            } else {
                (x.distancia.clone(), x.lazo.0.clone())
            })
            .collect()
    }
}

impl fmt::Display for Grafo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Aristas: ")?;
        for &Arista {
            lazo: (ref origen, ref destino),
            distancia: dis
        } in self.aristas.iter() {
            write!(f, "\n\t{}, {} : {}", origen, destino, dis)?;
        }
        Ok(())
    }
}
/*-----------------------------------------Ruta---------------------------------------*/
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ruta {
    pub verts: Vec<Vertice>,
}

impl Ruta {
    pub fn new(dato: Vec<Vertice>) -> Ruta {
        Ruta {
            verts: dato
        }
    }

    pub fn get_distancia(&self) -> u64 {
        if self.verts.is_empty() {
            0
        } else {
            match self.verts
                .last()
                .unwrap()
                .distancia
                .1 {
                Some(x) => x,
                None => 0
            }
        }
    }
}

impl fmt::Display for Ruta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ");
        for vert in self.verts.iter() {
            write!(f, "{} ", vert.id);
        }
        write!(f, "]");
        Ok(())
    }
}

/*--------------------------Dikstra-Distancia más corta------------------------*/
pub fn ruta_mas_corta(origen: &str, destino: &str, g: &Grafo) -> Option<Ruta> {
    let mut g = g.clone();

    let mut vertice_actual = Vertice::new("");

    {
        let mut temp = g.buscar_id_mut(origen);

        let temp = match temp {
            Some(ref mut vert) => vert,
            None => return None,
        };
        temp.distancia = (None, Some(0));
    }

    let mut visitado: HashSet<String> = HashSet::new();

    loop {
        if vertice_actual.id == destino.to_string() {
            return Some(ruta(&g, destino));
        }

        visitado.insert(vertice_actual.clone().id);

        let mut cola: Vec<Vertice> = g.clone()
            .vertices
            .into_iter()
            .filter(|x| !visitado.contains(&x.id) && !x.distancia.1.is_none())
            .collect();

        if cola.is_empty() {
            return None;
        }

        cola.sort();

        vertice_actual = cola.remove(0);

        for (dist, terminal) in g.clone().buscar_vecinos(&vertice_actual.id) {
            let objetivo = g.buscar_id_mut(&terminal).unwrap();

            let t_dist = vertice_actual.clone()
                .distancia
                .1
                .unwrap() + dist;

            let actualizar_objetivo = Vertice {
                distancia: (Some(vertice_actual.clone().id),
                            Some(t_dist)),
                ..objetivo.clone()
            };

            *objetivo = if &actualizar_objetivo < objetivo {
                actualizar_objetivo.clone()
            } else {
                objetivo.clone()
            };
        }
    }
}

fn ruta(g: &Grafo, id: &str) -> Ruta {
    let mut ruta = Vec::new();

    let mut actual_id = id;

    loop {
        let actual = match g.vertices.iter().find(|&x| x.id == actual_id.to_string()) {
            Some(vert) => vert,
            None => break,
        };

        ruta.insert(0, actual.clone());

        match actual.distancia.0 {
            Some(ref hijo) => actual_id = hijo,
            None => break,
        }
    }

    Ruta { verts: ruta }
}

fn main() {
    let mut grafo = Grafo::new(
        vec![
            (4, "A", "B"),
            (3, "A", "C"),
            (8, "B", "E"),
            (12, "C", "D"),
            (4, "C", "F"),
            (20, "D", "G"),
            (15, "D", "H"),
            (17, "E", "G"),
            (22, "F", "H"),
            (9, "G", "H"),
        ]);

    let ruta = ruta_mas_corta("A", "H", &grafo).unwrap();
    println!("{:?}", ruta.to_string());
    print!("{}", ruta.get_distancia());
}
