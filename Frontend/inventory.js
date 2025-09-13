
function isObject(value){

    if (typeof value === 'object'){
        Object.keys(value).forEach(item =>{ if (typeof value[item] == 'object'){Object.keys(item).forEach(objectInobject => {
           console.log(`This is object in object: ${item}:${Object.values(value[item])}`) 
        });} else {console.log(`type of ${item}: ${typeof value[item]}`); console.log(`${item}:: ${value[item]}`)}})
            
        
    }
    else return value

}


document.addEventListener("DOMContentLoaded",()=>

    {
        document.getElementById("slider").addEventListener('click',()=>
            {
                if (document.getElementById("slider").checked == true)
                    {
                        document.querySelector(".slider.round").innerHTML = "Display"
                        document.getElementById("hidden-table").style.display = "None"
                        document.getElementById("items-box").style.display = "flex"
                    } 
                else 
                    {
                        document.querySelector(".slider.round").textContent = "Table"
                        document.getElementById("items-box").style.display = "None"
                        document.getElementById("hidden-table").style.display = "block"
                    }
            })
        
        document.getElementById("submitButton").addEventListener("click",(event)=>{console.log("Submitted")})
        document.getElementById("closeButton").addEventListener("click",(event)=>
        {
        event.target.parentElement.style.display = "none"
        document.querySelectorAll("#items-wrapper").forEach(elem=>{elem.style.opacity = 1});
        document.querySelectorAll("#editButton").forEach(elem=>{elem.disabled =false})
        document.querySelectorAll(".toggle-button.selected").forEach(elem=>{elem.className= "toggle-button"})
        console.log("Tried to close")    
        })
        fetch("https://home-inventory-bml1.onrender.com/item")
        .then(Response =>Response.json())
        .then(data => 
            {
                data.forEach(element => 
                    {
                        
                        let item_name = element.item_name
                        let id = element._id.$oid
                        let category = element.category
                        let method_measure = element.method_measure
                        let date =new Date(Number(element.date.$date.$numberLong))
                        let quantity = element.quantity
                        let unit_price = element.unit_price


                        console.log(`${date} -- ${element.date.$date.$numberLong}`)

                        //creating elements


                        //table display
                        let table_row = document.createElement("tr")

                        let th_name = document.createElement("th")
                        th_name.textContent =item_name

                        let th_category = document.createElement("th")
                        th_category.textContent = category

                        let th_date = document.createElement("th")
                        th_date.textContent = date

                        let th_quantity = document.createElement("th")
                        th_quantity.textContent = `${quantity} ${method_measure}`

                        let th_unit_price = document.createElement("th")
                        th_unit_price.textContent = `$${unit_price.$numberDecimal}`


                        //Box display
                        let box =document.createElement("div")
                        box.style.borderColor="Black"
                        box.style.border = "1 px solid black"
                        box.className = "items-wrapperheader"
                        box.id = "items-wrapper"

                        let box_header = document.createElement("div")
                        box_header.id = box.id+"header"
                        
                        
                        let iNameElem = document.createElement("p")
                        iNameElem.textContent = item_name
                        iNameElem.id = "nameElem"
                        
                        
                        let idElem = document.createElement('p')
                        idElem.textContent =id
                        idElem.id = "id"
                        idElem.className ="id"
                        
                        let categoryElem = document.createElement("p")
                        categoryElem.textContent = category
                        categoryElem.id = "categoryElem"

                        let measureDiv = document.createElement("div")
                        measureDiv.id ="measureDiv"


                        let section1Div = document.createElement("section")
                        let section2Div = document.createElement("section")
                        measureDiv.appendChild(section1Div)
                        measureDiv.appendChild(section2Div)

                        let method_measureElem =document.createElement('p')
                        method_measureElem.textContent = method_measure
                        method_measureElem.id = "method_measure"
                        section2Div.appendChild(method_measureElem)


                        let quantityElem =document.createElement('p')
                        quantityElem.textContent = quantity
                        quantityElem.id = "quantityElem"
                        section1Div.appendChild(quantityElem)


                        let unit_priceElem = document.createElement("p")
                        unit_priceElem.textContent = "$"+unit_price.$numberDecimal
                        unit_priceElem.id = "unit_priceElem"

                        let dateElem = document.createElement("p")
                        dateElem.textContent = date
                        dateElem.id = "dateElem"

                        let button_div =document.createElement("div")
                        button_div.id ="buttonDiv"

                        let buttonSection1 = document.createElement("section")
                        buttonSection1.id="buttonSection"

                        let buttonSection2 = document.createElement("section")
                        buttonSection2.id="buttonSection"
                        button_div.appendChild(buttonSection1)
                        button_div.appendChild(buttonSection2)

                        let edit_button = document.createElement("button")
                        edit_button.id="editButton"
                        edit_button.textContent="Edit"
                        edit_button.type = "button"
                        
                        
                        let delete_button = document.createElement("button")
                        delete_button.id = "deleteButton"
                        delete_button.textContent = "Delete"
                        
                        console.log(element)

                        //putting together the table

                        table_row.appendChild(th_name)
                        table_row.appendChild(th_category)
                        table_row.appendChild(th_date)
                        table_row.appendChild(th_quantity)
                        table_row.appendChild(th_unit_price)
                        document.getElementById("inventory-table").appendChild(table_row)


                        //putting together the box display
                        box.appendChild(iNameElem)
                        box.appendChild(idElem)
                        box.appendChild(categoryElem)
                        box.appendChild(measureDiv)
                        box.appendChild(unit_priceElem)
                        box.appendChild(dateElem)
                        buttonSection1.appendChild(edit_button)
                        buttonSection2.appendChild(delete_button)
                        box.appendChild(button_div)
                   
                        document.getElementById("items-box").appendChild(box)
                        edit_button.addEventListener("click",(event)=>
                            {
                            
                                
                                //fetch(`https://home-inventory-bml1.onrender.com/item/${event.target.parentElement.parentElement.parentElement.querySelector('[id]').textContent}`)
                                document.getElementById("popup").style.display = "block"
                                //event.target.parentElement.parentElement.parentElement.querySelector("#items-wrapper").style.opacity = .5
                                document.querySelectorAll("#items-wrapper").forEach(elem=>{elem.style.opacity = .5})
                                //let ids = ["id","nameInput","CategoryInput","selectInput","amountInput","timeInput","submitButton","closeButton"]
                                document.getElementById("idInput").value = event.target.parentElement.parentElement.parentElement.querySelector('.id').textContent
                                document.getElementById("idLabel2").innerHTML = event.target.parentElement.parentElement.parentElement.querySelector('.id').textContent
                                
                                //nameElem
                                document.getElementById("nameInput").value = event.target.parentElement.parentElement.parentElement.querySelector('#nameElem').textContent
                                
                                //categoryElem
                                //document.querySelector("#categories").querySelectorAll("div").forEach((item)=>{if (console.log(item.dataset.value))})
                                event.target.parentElement.parentElement.parentElement.querySelector("#categoryElem").textContent.split(",").map((itemo)=>{console.log(itemo);document.querySelector(`[data-value=${itemo}]`).className="toggle-button selected"})
                                //document.getElementById("CategoryInput").value = event.target.parentElement.parentElement.parentElement.querySelector("#categoryElem").textContent
                                console.log( event.target.parentElement.parentElement.parentElement.querySelector("#categoryElem").textContent)

                                //method_measure
                                document.getElementById("selectInput").value = event.target.parentElement.parentElement.parentElement.querySelector('#method_measure').textContent

                                //quantityElem
                                document.getElementById("amountInput").value = event.target.parentElement.parentElement.parentElement.querySelector("#quantityElem").textContent


                                //unit_priceElem
                                document.getElementById("priceInput").value = event.target.parentElement.parentElement.parentElement.querySelector("#unit_priceElem").textContent.substring(1,event.target.parentElement.parentElement.parentElement.querySelector("#unit_priceElem").textContent.length)

                                //dateElem
                                document.getElementById("timeInput").value = event.target.parentElement.parentElement.parentElement.querySelector("#dateElem").textContent
                                document.querySelectorAll("#editButton").forEach(elem=>{elem.disabled =true})
                                console.log(`X${scrollX}  Y${scrollY}`)
                                document.getElementById("popup").style.top = `${window.scrollY}px`;
                                document.getElementById("popup").style.left = `${window.scrollX}px`;

                            })
                        
                       //document.querySelectorAll("#items-wrapper").forEach((elem, index)=>{dragElement(elem);elem.style.top =`${index*600+100}px`});
                       
                       Object.keys(element).forEach(key => {
                        console.log(`${key}: ${isObject(element[key])}`);
                    })
                    }
                );
            }
        )
       
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


    
function dragElement(elmnt) {
  var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
  if (document.getElementById(elmnt.id + "header")) {
    /* if present, the header is where you move the DIV from:*/
    document.getElementById(elmnt.id + "header").onmousedown = dragMouseDown;
  } else {
    /* otherwise, move the DIV from anywhere inside the DIV:*/
    elmnt.onmousedown = dragMouseDown;
  }

  function dragMouseDown(e) {
    e = e || window.event;
    e.preventDefault();
    // get the mouse cursor position at startup:
    pos3 = e.clientX;
    pos4 = e.clientY;
    document.onmouseup = closeDragElement;
    // call a function whenever the cursor moves:
    document.onmousemove = elementDrag;
  }

  function elementDrag(e) {
    e = e || window.event;
    e.preventDefault();
    // calculate the new cursor position:
    pos1 = pos3 - e.clientX;
    pos2 = pos4 - e.clientY;
    pos3 = e.clientX;
    pos4 = e.clientY;
    // set the element's new position:
    console.log(elmnt.style.top)
    elmnt.style.top = (elmnt.offsetTop - pos2) + "px";
    elmnt.style.left = (elmnt.offsetLeft - pos1) + "px";
  }

  function closeDragElement() {
    /* stop moving when mouse button is released:*/
    document.onmouseup = null;
    document.onmousemove = null;
  }
}


