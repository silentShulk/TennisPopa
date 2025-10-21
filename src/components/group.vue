<template>
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
</template>



</sctipt>



<style>
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

.name {
    background: transparent;
    border-top: 1px solid black;
    border-bottom: 1px solid black;
  }
</style>