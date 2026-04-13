<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  // 1. EL PLANO TÉCNICO: Le decimos a TypeScript la forma exacta de los datos que manda Rust
  interface AnalysisResult {
    gramatical: string;
    gramatical_count: number;
    adjustment: number;
    poetica: string;
    count: number;
    kind: string;
    ritmo: string;
    ritmo_numeros?: string;
    terminacion: string;
    letraRima?: string; // Es opcional porque se la agregamos dinámicamente aquí en Svelte
  }

  let text = "";
  // Etiquetamos la variable final
  let results: AnalysisResult[] = [];

  async function handleInput() {
    const lines = text.split('\n').filter(l => l.trim() !== "");
    
    // 2. CONEXIÓN TIPADA: Le avisamos a invoke() que devolverá un objeto AnalysisResult
    let rawResults = await Promise.all(
      lines.map(line => invoke<AnalysisResult>("analyze_line", { line }))
    );

    // 3. DICCIONARIO DECLARADO: Le decimos que las llaves son texto y los valores números
    let freq: Record<string, number> = {};
    rawResults.forEach(r => {
      if (r.terminacion) {
        freq[r.terminacion] = (freq[r.terminacion] || 0) + 1;
      }
    });

    // 4. MAPA DECLARADO: Textos apuntan a Textos
    let rhymeMap = new Map<string, string>();
    let nextCharCode = 97;

    results = rawResults.map(r => {
      let t = r.terminacion;
      let letraRima = '-'; 

      if (t && freq[t] > 1) {
        if (!rhymeMap.has(t)) {
          rhymeMap.set(t, String.fromCharCode(nextCharCode++));
        }
        
        // El "!" al final le asegura al linter que el valor no será nulo
        let baseLetter = rhymeMap.get(t)!; 
        
        letraRima = r.count >= 9 ? baseLetter.toUpperCase() : baseLetter;
      }

      return { ...r, letraRima };
    });
  }
</script>

<main class="app-wrapper">
  <div class="container">
    <div class="header">
      <h1>ül</h1>
    </div>

    <textarea 
      bind:value={text} 
      on:input={handleInput} 
      placeholder=" Escribe o pega tu poema aquí..."
    ></textarea>

    <div class="results-list">
      {#each results as res}
        <div class="card">
          
          <div class="top-row">
            <div class="poetica-container">
              {#each res.poetica.split('-') as syl, i}
                <div class="syllable-col">
                  <span class="syl-text">{syl}</span>
                  <span class="syl-circle">{res.ritmo.split(' ')[i] || '◯'}</span>
                </div>
                {#if i !== res.poetica.split('-').length - 1}
                  <span class="syl-hyphen">-</span>
                {/if}
              {/each}
            </div>
            
            <div class="metric-equation">
              <span class="base-count">{res.count - res.adjustment}</span>
              <span class="adjustment {res.adjustment >= 0 ? 'plus' : 'minus'}">
                {res.adjustment >= 0 ? '+' : ''}{res.adjustment}
              </span>
              <span class="equals">=</span>
              <span class="final-badge">{res.count}</span>
            </div>
          </div>
          
          <div class="bottom-row">
            <span class="gramatical">
              Gramatical: {res.gramatical} <strong>= {res.gramatical_count}</strong>
            </span>
            
            <div class="metric-info">
              {#if res.ritmo_numeros}
                <span class="rhythm-tag">Ritmo: {res.ritmo_numeros}</span>
              {/if}
              
              <span class="tag {res.kind}">
                {res.kind}
              </span>

              {#if res.terminacion}
                <span class="tag" style="background: #333; color: #aaa;">
                  {res.terminacion}
                </span>
              {/if}
              
              <span class="rhyme-letter" class:suelto={res.letraRima === '-'}>
                {res.letraRima}
              </span>

              {#if res.kind === 'doble acento'}
                <div class="tooltip-container">
                  <span class="icon-help">?</span>
                  <div class="tooltip-text">
                    Según la RAE, los adverbios en "-mente" conservan dos acentos (adjetivo + sufijo).
                    Para la métrica, se consideran palabras graves (+0).
                  </div>
                </div>
              {/if}
            </div>
          </div>
          
        </div> {/each}
    </div>
  </div>
</main>

<style>
  /* BLINDAJE DE FONDO GLOBAL */
  :global(html, body) { 
    background-color: #0f0f0f !important; 
    margin: 0; 
    padding: 0;
    min-height: 100vh;
  }

  .app-wrapper {
    min-height: 100vh;
    background-color: #0f0f0f;
    color: #cfcfcf;
    font-family: 'Inter', sans-serif;
  }

  /* EL RESTO DE TUS ESTILOS INTACTOS */
  .container { max-width: 900px; margin: 0 auto; padding: 2rem; }
  .header { text-align: center; margin-bottom: 2rem; }
  h1 { color: #00d4ff; font-size: 3.5rem; margin: 0; letter-spacing: -2px; }

  textarea { 
    width: 100%; height: 140px; background: #141414; color: #fff; 
    border: 1px solid #222; border-radius: 12px; padding: 1.2rem; 
    font-size: 1.2rem; outline: none; margin-bottom: 2rem;
    transition: border-color 0.2s;
    box-sizing: border-box; /* El ajuste de tolerancia que le agregamos */
  }
  textarea:focus { border-color: #00d4ff; }

  .card { 
    background: #0a0a0a; border: 1px solid #2a2a2a; border-radius: 8px; 
    padding: 1rem; margin-bottom: 0.75rem;
    box-shadow: 0 4px 15px rgba(0,0,0,0.2);
  }

  .top-row { 
    display: flex; 
    justify-content: space-between; 
    align-items: flex-start; /* Alinea todo hacia arriba para evitar saltos raros */
    flex-wrap: wrap; /* Permite que la ecuación baje si el verso es MUY largo */
    gap: 15px;
    margin-bottom: 1rem;
  }
  .poetica { font-size: 1.6rem; font-family: 'Crimson Pro', serif; color: #fff; letter-spacing: 0.5px; }
  
.metric-equation {
    display: flex;
    align-items: center;
    gap: 8px;
    background: #111;
    padding: 6px 14px;
    border-radius: 30px;
    border: 1px solid #333;
    width: fit-content; /* ¡LA CLAVE! Evita que la píldora se estire a todo el ancho */
    height: fit-content;
    margin-left: auto; /* Si el verso es muy largo y la obliga a bajar, se alineará a la derecha */
    white-space: nowrap; /* Protege los números para que no se separen en dos líneas */
  }
  
  .base-count { font-size: 1.2rem; color: #aaa; font-weight: bold; }
  .adjustment { font-size: 1.1rem; font-weight: bold; }
  .adjustment.plus { color: #00ff88; } 
  .adjustment.minus { color: #ff4444; } 
  .equals { color: #666; font-weight: bold; }
  
  .final-badge { 
    background: #00d4ff; color: #000; font-weight: 800; font-size: 1.2rem;
    width: 35px; height: 35px; border-radius: 50%; 
    display: flex; align-items: center; justify-content: center;
    box-shadow: 0 0 10px rgba(0, 212, 255, 0.3);
  }

  .bottom-row { 
      display: flex; 
      justify-content: space-between; 
      align-items: center;
      padding-top: 0.6rem; 
      margin-top: 0.6rem; /* Añadimos un poco de aire */
      border-top: 1px solid #1a1a1a; /* La línea separadora sutil */
      font-size: 0.85rem;
 }
  
  .gramatical { color: #777; font-family: monospace; }
  .gramatical strong { color: #00d4ff; margin-left: 5px; }

  .tag { font-weight: bold; text-transform: uppercase; padding: 4px 10px; border-radius: 6px; background: #222; font-size: 0.8rem; letter-spacing: 1px;}
  .aguda { color: #00ff88; border-left: 3px solid #00ff88; }
  .grave { color: #888; border-left: 3px solid #888; }
  .esdrújula { color: #ff4444; border-left: 3px solid #ff4444; }
  .doble\ acento { color: #b066ff; border-left: 3px solid #b066ff; }

  /* --- ESTILOS DEL TOOLTIP RAE --- */
  .metric-info { display: flex; align-items: center; gap: 8px; }

  .tooltip-container {
    position: relative;
    display: inline-block;
    cursor: help;
  }

  .icon-help {
    background: #b066ff;
    color: #000;
    border-radius: 50%;
    width: 18px;
    height: 18px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    font-size: 0.75rem;
    font-weight: 900;
    font-family: monospace;
    opacity: 0.8;
    transition: opacity 0.2s;
  }
  
  .tooltip-container:hover .icon-help { opacity: 1; }

  .tooltip-text {
    visibility: hidden;
    width: 240px;
    background-color: #1a1a1a;
    color: #e0e0e0;
    text-align: left;
    border-radius: 8px;
    padding: 10px 14px;
    position: absolute;
    z-index: 10;
    bottom: auto;
    top:130%;
    right: -10px; /* Anclado hacia la izquierda para que no se salga de la tarjeta */
    font-size: 0.85rem;
    border: 1px solid #b066ff;
    box-shadow: 0 4px 15px rgba(176, 102, 255, 0.15);
    opacity: 0;
    transition: opacity 0.3s, transform 0.3s;
    transform: translateY(-5px);
    font-weight: normal;
    text-transform: none;
    line-height: 1.4;
    letter-spacing: normal;
  }

  /* La flechita del cuadro de diálogo */
  .tooltip-text::after {
    content: "";
    position: absolute;
    bottom: 100%;
    right: 14px;
    border-width: 6px;
    border-style: solid;
    border-color: transparent transparent #b066ff transparent;
  }

  .tooltip-container:hover .tooltip-text {
    visibility: visible;
    opacity: 1;
    transform: translateY(0);
  }

  /* Contenedor principal de la frase */
  .poetica-container { 
    display: flex; 
    flex-wrap: wrap; 
    align-items: flex-start; 
    flex: 1; /* Le dice que ocupe todo el espacio de texto posible */
    min-width: 250px; /* Evita que colapse en pantallas pequeñas */
  }

  /* La mini-columna que sostiene la sílaba y su círculo */
  .syllable-col {
    display: flex;
    flex-direction: column;
    align-items: center; /* Esto centra el círculo bajo la sílaba */
    gap: 6px; /* Separación entre la letra y el círculo */
  }

  /* El texto de la sílaba */
  .syl-text {
    font-size: 1.4rem;
    font-family: 'Crimson Pro', serif;
    color: #fff;
    line-height: 1;
  }

  /* El círculo del ritmo */
  .syl-circle {
    font-size: 0.8rem;
    color: #00d4ff; /* Mantenemos tu color cian */
    opacity: 0.7;
    line-height: 0.5;
  }

  /* El guion separador */
  .syl-hyphen {
    font-size: 1.8rem;
    font-family: 'Crimson Pro', serif;
    color: #444; /* Un tono más oscuro para que no robe atención */
    margin: 0 4px; /* Margen a los lados del guion */
    line-height: 1;
  }

  .rhyme-letter {
    font-family: 'Space Mono', monospace;
    font-weight: 800;
    font-size: 0.75rem;
    color: #00d4ff;
    background: rgba(0, 212, 255, 0.1);
    padding: 1px 8px;
    border-radius: 4px;
    border: 1px solid rgba(0, 212, 255, 0.3);
  }

  /* Si el verso es suelto, lo atenuamos */
  .rhyme-letter.suelto {
    color: #444;
    background: transparent;
    border-color: #222;
  }

</style>