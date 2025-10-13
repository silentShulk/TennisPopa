<template>
  <div class="page">
    <h1>Pagina 1</h1>
    <p>Questo è il contenuto completo della prima pagina aggiuntiva.</p>
    
    <div class="content-section">
      <h2>Contenuto Specifico Pagina 1</h2>
      <p>Qui puoi aggiungere tutto il contenuto che desideri per questa pagina.</p>
      <ul>
        <li>Elemento 1 della lista</li>
        <li>Elemento 2 della lista</li>
        <li>Elemento 3 della lista</li>
      </ul>
    </div>
    
    <div class="back-navigation">
      <router-link to="/" class="back-button">← Torna alla Home</router-link>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Page1'
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