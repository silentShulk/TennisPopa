<template>
  <div id="app">
    <div class="sidebar">
      <div class="sidebar-header">
        <h2>Torneo Sociale</h2>
      </div>
      <nav class="sidebar-nav">
        <router-link 
          to="/" 
          class="nav-link"
          :class="{ active: $route.path === '/' }"
        >
          Creazione Form
        </router-link>
        <router-link 
          to="/GetFormResponses" 
          class="nav-link"
          :class="{ active: $route.path === '/GetFormResponses' }"
        >
          Ottieni Riposte Form
        </router-link>
        <router-link 
          to="/Groups" 
          class="nav-link"
          :class="{ active: $route.path === '/Groups' }"
        >
          Gironi
        </router-link>
        <router-link 
          to="/Player" 
          class="nav-link"
          :class="{ active: $route.path === '/Player' }"
        >
          Anagrafiche
        </router-link>
        <router-link 
          to="/Schedule" 
          class="nav-link"
          :class="{ active: $route.path === '/Schedule' }"
        >
          Orari Campi
        </router-link>
        <router-link 
          to="/Matches" 
          class="nav-link"
          :class="{ active: $route.path === '/Matches' }"
        >
          Partite
        </router-link>
      </nav>
    </div>

    <!-- Contenuto principale -->
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<script>
export default {
  name: 'App'
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

#app {
  display: flex;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

/* Sidebar Styles */
.sidebar {
  width: 250px;
  background: linear-gradient(180deg, #2c3e50 0%, #34495e 100%);
  color: white;
  height: 100vh;
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 10px rgba(0, 0, 0, 0.1);
}

.sidebar-header {
  padding: 25px 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.sidebar-header h2 {
  font-size: 1.3rem;
  font-weight: 600;
}

.sidebar-nav {
  padding: 20px 0;
  flex: 1;
}

.nav-link {
  display: block;
  padding: 15px 25px;
  color: #bdc3c7;
  text-decoration: none;
  border-left: 4px solid transparent;
  transition: all 0.3s ease;
  font-size: 1rem;
  margin: 5px 0;
}

.nav-link:hover {
  background: rgba(255, 255, 255, 0.1);
  color: white;
  border-left-color: #3498db;
}

.nav-link.active {
  background: rgba(52, 152, 219, 0.2);
  color: white;
  border-left-color: #3498db;
  font-weight: 600;
}

/* Contenuto principale */
.main-content {
  flex: 1;
  background: #ecf0f1;
  height: 100vh;
  overflow-y: auto;
  padding: 0;
}
</style>