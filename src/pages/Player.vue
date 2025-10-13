<template>
  <div class="page">
    <h1>Pagina 2</h1>
    <p>Questo è il contenuto completo della seconda pagina aggiuntiva.</p>
    
    <div class="content-section">
      <h2>Contenuto Specifico Pagina 2</h2>
      <p>Qui puoi aggiungere contenuti diversi per la seconda pagina.</p>
      
      <div class="feature-grid">
        <div class="feature">
          <h3>Funzionalità A</h3>
          <p>Descrizione della funzionalità A</p>
        </div>
        <div class="feature">
          <h3>Funzionalità B</h3>
          <p>Descrizione della funzionalità B</p>
        </div>
        <div class="feature">
          <h3>Funzionalità C</h3>
          <p>Descrizione della funzionalità C</p>
        </div>
      </div>
    </div>
    
    <div class="back-navigation">
      <router-link to="/" class="back-button">← Torna alla Home</router-link>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Page2'
}
</script>

<style scoped>
.page {
  max-width: 800px;
  margin: 0 auto;
}

.content-section {
  background: white;
  padding: 30px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  margin: 30px 0;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.feature {
  padding: 20px;
  background: #f8f9fa;
  border-radius: 6px;
  text-align: center;
}

.feature h3 {
  margin: 0 0 10px 0;
  color: #007acc;
}

.back-navigation {
  margin-top: 40px;
  text-align: center;
}

.back-button {
  display: inline-block;
  padding: 12px 24px;
  background-color: #6c757d;
  color: white;
  text-decoration: none;
  border-radius: 6px;
  transition: background-color 0.3s;
}

.back-button:hover {
  background-color: #545b62;
}
</style>