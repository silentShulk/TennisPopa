<template>
  <div id="app">
    <nav v-if="$route.path !== '/'" class="breadcrumb">
      <router-link to="/">Home</router-link>
      <span v-if="$route.name"> › {{ $route.name }}</span>
    </nav>
    
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
#app {
  font-family: Arial, sans-serif;
  padding: 20px;
}

.breadcrumb {
  margin-bottom: 20px;
  padding: 10px;
  background-color: #f8f9fa;
  border-radius: 4px;
}

.breadcrumb a {
  color: #007acc;
  text-decoration: none;
}

.breadcrumb a:hover {
  text-decoration: underline;
}

.main-content {
  min-height: 400px;
}
</style>