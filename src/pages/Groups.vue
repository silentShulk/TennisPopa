<script>
import Group from '../components/Group.vue';
export default {
  name: 'SomePage',
  components: { Group },
  data() {
    return {
      anyModalOpen: false
    };
  }
};
</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <h1>Gironi Torneo</h1>
      <p>Gestisci e visualizza i gironi del torneo per categoria</p>
    </div>
    <div class="category-section">
      <div class="category-card">
        <label for="categoriaComboBox" class="category-label">Seleziona Categoria:</label>
        <select id="categoriaComboBox" class="category-select">
          <option value="" selected disabled>Seleziona una categoria</option>
          <option value="A1">A1</option>
          <option value="A2">A2</option>
          <option value="B1">B1</option>
          <option value="B2">B2</option>
          <option value="C">C</option>
          <option value="D">D</option>
          <option value="E">E</option>
        </select>
      </div>
    </div>
    <div class="groups-section">
      <div class="groups-grid">
        <div class="group-wrapper">
          <Group :any-modal-open="anyModalOpen" @modal-opened="anyModalOpen = true" @modal-closed="anyModalOpen = false" />
        </div>
        <div class="group-wrapper">
          <Group :any-modal-open="anyModalOpen" @modal-opened="anyModalOpen = true" @modal-closed="anyModalOpen = false" />
        </div>
        <div class="group-wrapper">
          <Group :any-modal-open="anyModalOpen" @modal-opened="anyModalOpen = true" @modal-closed="anyModalOpen = false" />
        </div>
        <div class="group-wrapper">
          <Group :any-modal-open="anyModalOpen" @modal-opened="anyModalOpen = true" @modal-closed="anyModalOpen = false" />
        </div>
        <div class="group-wrapper">
          <Group :any-modal-open="anyModalOpen" @modal-opened="anyModalOpen = true" @modal-closed="anyModalOpen = false" />
        </div>
        <div class="group-wrapper">
          <Group :any-modal-open="anyModalOpen" @modal-opened="anyModalOpen = true" @modal-closed="anyModalOpen = false" />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  padding-bottom: 2rem;
}

.page-header {
  background: rgba(255, 255, 255, 0.95);
  padding: 2rem;
  text-align: center;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);
}

.page-header h1 {
  color: #2c3e50;
  margin: 0 0 0.5rem 0;
  font-size: 2.8rem;
  font-weight: 300;
  letter-spacing: -0.5px;
}

.page-header p {
  color: #7f8c8d;
  font-size: 1.1rem;
  margin: 0;
}

.category-section {
  padding: 2rem;
  display: flex;
  justify-content: center;
}

.category-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  padding: 1.5rem 2rem;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  display: flex;
  align-items: center;
  gap: 1rem;
  min-width: 300px;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.category-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
}

.category-label {
  color: #2c3e50;
  font-size: 1.2rem;
  font-weight: 600;
  white-space: nowrap;
}

.category-select {
  flex: 1;
  padding: 0.75rem 1rem;
  border: 2px solid #e9ecef;
  border-radius: 10px;
  font-size: 1rem;
  background: white;
  color: #2c3e50;
  cursor: pointer;
  transition: border-color 0.3s ease, box-shadow 0.3s ease;
  appearance: none;
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e");
  background-position: right 0.5rem center;
  background-repeat: no-repeat;
  background-size: 1.5em 1.5em;
  padding-right: 2.5rem;
}

.category-select:hover {
  border-color: #4facfe;
  box-shadow: 0 0 0 3px rgba(79, 172, 254, 0.1);
}

.category-select:focus {
  outline: none;
  border-color: #4facfe;
  box-shadow: 0 0 0 3px rgba(79, 172, 254, 0.2);
}

.groups-section {
  padding: 0 2rem 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.groups-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 2rem;
  padding: 1rem 0;
}

.group-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.group-wrapper:hover {
  transform: translateY(-4px);
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .category-section {
    padding: 1rem;
  }

  .category-card {
    min-width: auto;
    flex-direction: column;
    gap: 0.75rem;
    text-align: center;
  }

  .groups-grid {
    grid-template-columns: 1fr;
    gap: 1.5rem;
    padding: 0.5rem;
  }

  .page-header h1 {
    font-size: 2.2rem;
  }
}

@media (max-width: 480px) {
  .groups-section {
    padding: 0 1rem 1rem;
  }
}
</style>