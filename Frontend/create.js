
document.addEventListener("DOMContentLoaded",()=>{
    let time= document.getElementById("timeInput")
    time.value = new Date().getTime()
    const input = document.getElementById('priceInput');
    

    // Format to two decimal places
    function formatDecimal() {
        let value = parseFloat(input.value) || 0;
        input.value = value.toFixed(2);
        
    }

    // Trigger formatting on input, blur, and change events
    input.addEventListener('input', formatDecimal);
    input.addEventListener('blur', formatDecimal);
    input.addEventListener('change', formatDecimal);

    // Initialize with 0.00
    input.value = '0.00';

    })

function toggleSelection(element) 
        {
            element.classList.toggle('selected');
        }

function getSelectedCategories() 
    {
        const buttons = document.querySelectorAll('.toggle-button.selected');
        const selectedOptions = Array.from(buttons).map(button => button.getAttribute('data-value'));
        console.log(selectedOptions);
    }


