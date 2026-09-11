# Raytracer: Cubo
# Gráficas por Computadora
# Hugo Méndez Lee - 241265

Una implementación de un **motor de trazado de rayos (Raytracer)** escrito desde cero en Rust. Este proyecto base es capaz de renderizar geometría 3D pura, calcular intersecciones matemáticas de rayos, y procesar iluminación dinámica con sombras utilizando la librería `minifb` para pintar píxel por píxel en un _framebuffer_.

## 📸 Demo

*(Añade aquí una captura de pantalla o GIF de tu cubo renderizado)*
<img width="800" height="600" alt="Raytracer Demo" src="https://github.com/user-attachments/assets/d337e6c7-f819-4ed7-921e-d57fe7dd73c9" />

## 🚀 Características Principales

* **Renderizado a Mano:** Todo se dibuja trazando rayos (`cast_ray`) matemáticamente sin utilizar APIs gráficas tradicionales (como OpenGL o Vulkan).
* **Geometría AABB:** Las intersecciones espaciales se evalúan calculando colisiones contra Cajas Delimitadoras Alineadas a los Ejes (Axis-Aligned Bounding Boxes). Esto permite renderizar cubos matemáticamente perfectos.
* **Sistema de Iluminación de Phong:** Cuenta con luz ambiental, luz difusa y luz especular, logrando que el objeto adquiera un sombreado 3D realista basado en las normales de cada cara.
* **Sombras Dinámicas (Cast Shadows):** Detección de colisión de rayos hacia la fuente de luz para proyectar sombras sólidas en los demás objetos de la escena.
* **Cámara Orbital:** Puedes navegar libremente alrededor del cubo utilizando las teclas de flechas, actualizando el render en tiempo real.

## 🎨 Explorando Texturas (Rama `texture`)

Este repositorio cuenta con una rama paralela llamada `texture` donde la implementación base se expande añadiendo soporte para leer, muestrear y mapear imágenes PNG a las distintas caras del cubo (Multi-texturing).

Para cambiarte a esa rama y probar el cubo texturizado, puedes ejecutar en tu terminal:

```bash
git checkout texture
cargo run --release
```

## 🖥️ Especificaciones Técnicas

* **Librería de Gráficos:** [`minifb`](https://crates.io/crates/minifb) para la creación de la ventana nativa y el control directo del buffer de memoria de video.
* **Matemáticas y Vectores:** [`nalgebra-glm`](https://crates.io/crates/nalgebra-glm) para facilitar el álgebra lineal, normalizaciones y cambios de base de la cámara.
* **Resolución de la Ventana:** 800x600 píxeles.
* **Lenguaje:** Rust (Edición 2024).

## 📦 Escena Incluida

La escena principal está enfocada en probar el modelo de sombreado y geometría:

### 1. Cubo Principal
Un cubo color naranja mate (`Color::new(255, 140, 0)`) situado en el origen del mundo virtual, reaccionando dinámicamente a la luz y a tu ángulo de visión.

### 2. Suelo o Base
Un cuerpo sólido y ancho de color gris posicionado justo debajo del cubo principal. Su única finalidad es recibir y mostrar de forma clara la sombra proyectada por el cubo central.

## 🛠 Instalación y Ejecución

Asegúrate de tener instalado [Rust y Cargo](https://rustup.rs/). 

1. Clona este repositorio o abre la carpeta del proyecto en tu terminal.
2. Compila y ejecuta el código con el siguiente comando:

```bash
cargo run --release
```

**Controles:**
- **Flechas del teclado (⬅️ ➡️ ⬆️ ⬇️):** Orbitar la cámara alrededor de la escena.
- **ESC:** Cerrar la ventana y detener la simulación.