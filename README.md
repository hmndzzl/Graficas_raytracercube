# Raytracer: Cubo Texturizado
# Gráficas por Computadora
# Hugo Méndez Lee - 241265

Una implementación de un **motor de trazado de rayos (Raytracer)** escrito desde cero en Rust. Este proyecto es capaz de renderizar geometría 3D, calcular intersecciones precisas, y procesar iluminación, sombras dinámicas y mapeo de texturas, utilizando la librería `minifb` para escribir los píxeles al buffer.

## 📸 Demo

<img width="800" height="624" alt="RayTracer Demo" src="https://github.com/user-attachments/assets/3c837988-cdb6-4cb6-b050-ee0c8e2af122" />


## 🚀 Características Principales

* **Mapeo de Texturas (UV Mapping):** El cubo es capaz de desenvolverse matemáticamente y leer mapas de bits (PNGs). Cuenta con soporte avanzado de Multi-Texturing, asignando automáticamente una textura única a su cara superior (`top_material`) y otra para sus caras laterales.
* **Geometría AABB:** Las intersecciones espaciales con el cubo se evalúan calculando colisiones contra Cajas Delimitadoras Alineadas a los Ejes (Axis-Aligned Bounding Boxes), lo que permite deducir qué cara exacta (su Normal) fue impactada por el rayo.
* **Sistema de Iluminación y Sombreado:** Implementación de un modelo de iluminación de Phong que procesa luz ambiental, luz difusa, luz especular y proyecta sombras precisas cuando otros cuerpos se interponen.
* **Cámara Orbital:** Puedes navegar libremente alrededor del cubo utilizando las teclas de flechas (Arriba, Abajo, Izquierda, Derecha), permitiendo ver el modelo desde cualquier ángulo de forma dinámica.

## 🖥️ Especificaciones Técnicas

* **Librería de Gráficos:** [`minifb`](https://crates.io/crates/minifb) para la creación de la ventana y el control del frame buffer.
* **Librería de Imágenes:** [`image`](https://crates.io/crates/image) para decodificar, cargar y muestrear los colores (Sampling) de las texturas.
* **Matemáticas y Vectores:** [`nalgebra-glm`](https://crates.io/crates/nalgebra-glm) para álgebra lineal, productos punto, cruces y normalizaciones espaciales.
* **Resolución de la Ventana:** 800x600 píxeles.
* **Lenguaje:** Rust (Edición 2024).

## 📦 Escena Incluida

La escena principal que verás al ejecutar el código está compuesta por:

### 1. Cubo Texturizado
Ubicado en el origen del mundo virtual, este cuerpo demuestra la lectura de texturas desde el sistema de archivos (`assets/`):
- **Cara Superior:** `creaking_heart_top_awake.png`
- **Caras Laterales:** `creaking_heart_awake.png`

### 2. Suelo Mate
Un segundo cubo plano y ancho, de color gris oscuro, situado justo debajo del cubo principal. Su propósito es funcionar como suelo o pared de proyección para **hacer totalmente visibles las sombras** proyectadas dinámicamente por la fuente de luz del escenario.

## 🛠 Instalación y Ejecución

Asegúrate de tener instalado [Rust y Cargo](https://rustup.rs/). 

1. Clona este repositorio o abre la carpeta del proyecto en tu terminal.
2. Compila y ejecuta el código con el siguiente comando (la primera ejecución tomará unos segundos extra debido a las librerías matemáticas y de imágenes):

```bash
cargo run --release
```

**Controles:**
- **Flechas del teclado:** Orbitar la cámara alrededor del cubo.
- **ESC:** Cerrar la ventana y detener la simulación.
