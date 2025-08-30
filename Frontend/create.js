
document.addEventListener("DOMContentLoaded",()=>{
    let time= document.getElementById("timeInput")
    time.value = new Date().getTime()


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
