import Group from '@/components/group.vue';


<script>
export default {
  name: 'Page1'
}
</script>



<template>
  <div class="page-container">
    <div class="page-header">
      <h1>Gironi Torneo</h1>
    </div>
      <div class="categorySelection">
        <h1 class="categoria"> Categoria: </h1>
        <select id="categoriaComboBox">
          <option value="" selected disabled> Seleziona una categoria </option>
          <option value="A1"> A1 </option>
          <option value="A2"> A2 </option>
          <option value="B1"> B1 </option>
          <option value="B2"> B2 </option>
          <option value="C"> C </option>
          <option value="D"> D </option>
          <option value="E"> E </option>
        </select>
      </div>


      <div class="groups">
        <Group/>


        <div class="group">
          <p class="name"> Nome 1 </p>
          <div class="cell filled"></div>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell"></div>

          <p class="name"> Nome 2 </p>
          <div class="cell"></div>
          <div class="cell filled"></div>
          <div class="cell"></div>
          <div class="cell"></div>

          <p class="name"> Nome 3 </p>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell filled"></div>
          <div class="cell"></div>

          <p class="name"> Nome 4 </p>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell filled"></div>
        </div>


        <div class="group">
          <p class="name"> Nome 1 </p>
          <div class="cell filled"></div>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell"></div>

          <p class="name"> Nome 2 </p>
          <div class="cell"></div>
          <div class="cell filled"></div>
          <div class="cell"></div>
          <div class="cell"></div>

          <p class="name"> Nome 3 </p>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell filled"></div>
          <div class="cell"></div>

          <p class="name"> Nome 4 </p>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell filled"></div>
        </div>


        <div class="group">
          <p class="name"> Nome 1 </p>
          <div class="cell filled"></div>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell"></div>

          <p class="name"> Nome 2 </p>
          <div class="cell"></div>
          <div class="cell filled"></div>
          <div class="cell"></div>
          <div class="cell"></div>

          <p class="name"> Nome 3 </p>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell filled"></div>
          <div class="cell"></div>

          <p class="name"> Nome 4 </p>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell filled"></div>
        </div>


        <div class="group">
          <p class="name"> Nome 1 </p>
          <div class="cell filled"></div>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell"></div>

          <p class="name"> Nome 2 </p>
          <div class="cell"></div>
          <div class="cell filled"></div>
          <div class="cell"></div>
          <div class="cell"></div>

          <p class="name"> Nome 3 </p>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell filled"></div>
          <div class="cell"></div>

          <p class="name"> Nome 4 </p>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell filled"></div>
        </div>


        <div class="group">
          <p class="name"> Nome 1 </p>
          <div class="cell filled"></div>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell"></div>

          <p class="name"> Nome 2 </p>
          <div class="cell"></div>
          <div class="cell filled"></div>
          <div class="cell"></div>
          <div class="cell"></div>

          <p class="name"> Nome 3 </p>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell filled"></div>
          <div class="cell"></div>

          <p class="name"> Nome 4 </p>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell"></div>
          <div class="cell filled"></div>
        </div>
      </div>
  </div>
</template>



<style scoped>
select {
  font-size: 20px;

  border-radius: 10px;
}

.categorySelection {
  display: flex;
  gap: 10px;
  padding: 5px;
  margin: 5px;
}

  .groups {
    display: grid;              /* Uses CSS Grid layout for easy row/column definition */
    grid-template-columns: repeat(3, 300px);  /* 3 equal columns, each 100px wide */
    grid-template-rows: repeat(2, 250px);     /* 3 equal rows, each 100px tall */
    padding: 2px;                       /* Small padding inside the grid for clean edges */
    margin: 20px auto;
    gap: 2px;
  }

  .name {
    background: transparent;
    border-top: 1px solid black;
    border-bottom: 1px solid black;
  }

  /* Container for the entire 3x3 grid */
  .group {
    display: grid;
    grid-template-columns: repeat(5, 50px);   /* Unchanged: Inner 5 cols. */
    grid-template-rows: repeat(4, 50px);      /* Unchanged: Inner 4 rows. */
    gap: 0;                                   /* Unchanged from inner fix: For inner lines. */
    padding: 5px;                             /* Unchanged: Hugs inner lines. */
    margin: 20px auto;                        /* Unchanged. */
  }
        
  /* Base style for every individual cell */
  .cell {
    border: 2px solid black;              /* Red border around each cell */
    display: flex;                      /* Flexbox for potential centering of content */
    align-items: center;                /* Vertically centers any child elements */
    justify-content: center;            /* Horizontally centers any child elements */
    background-color: white;            /* White background for empty cells */
  }
        
  /* Style override for filled (oval) cells */
  .filled {
    background-color: black;              /* Solid red fill to match image ovals */
  }


.page-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #4facfe 0%, #00acb5 100%);
}

.page-header {
  background: rgba(255, 255, 255, 0.95);
  padding: 20px;
  text-align: center;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.page-header h1 {
  color: #2c3e50;
  margin-bottom: 10px;
  font-size: 2.5rem;
}

.page-content {
  padding: 40px;
}

.content-card {
  background: white;
  border-radius: 10px;
  padding: 30px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
  max-width: 1000px;
  margin: 0 auto;
}

.action-buttons {
  display: flex;
  gap: 15px;
  margin: 25px 0;
  flex-wrap: wrap;
}

.btn {
  padding: 12px 24px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.3s ease;
}

.btn-primary {
  background: #3498db;
  color: white;
}

.btn-secondary {
  background: #95a5a6;
  color: white;
}

.btn-success {
  background: #27ae60;
  color: white;
}

.btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-top: 30px;
}

.stat-card {
  background: #f8f9fa;
  padding: 25px;
  border-radius: 8px;
  text-align: center;
  border-top: 4px solid #e74c3c;
}

.stat-card h3 {
  color: #7f8c8d;
  margin-bottom: 10px;
  font-size: 1rem;
}

.stat-number {
  font-size: 2.5rem;
  font-weight: bold;
  color: #2c3e50;
}
</style>