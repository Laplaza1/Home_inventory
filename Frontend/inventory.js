function setCookie(name, value, daysToExpire) {
    const date = new Date();
    date.setTime(date.getTime() + (daysToExpire * 24 * 60 * 60 * 1000)); // Convert days to milliseconds
    const expires = "expires=" + date.toUTCString();
    document.cookie = `${name}=${value}; ${expires}; path=/`;
}



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


        const screenWidth = window.screen.width;
        const screenHeight = window.screen.height;

        console.log(`Screen Width: ${screenWidth}px, Screen Height: ${screenHeight}px`);

        //sumbit button
        // document.getElementById("popup").addEventListener("submit",(event)=>
        //     {
        //         console.log("form data")
        //         let formdata = new FormData(event.target)
        //         console.log(formdata)
        //     })

        // document.querySelector('form').addEventListener('submit', async (e) => 
        //     {
        //         e.preventDefault();
        //         const formData = new FormData(e.target);
        //         console.log(formData)
        //         console.log(e.target)
        //     })
        document.getElementById("searchInput").addEventListener("input",(s)=>
            {
                let search = document.getElementById("searchInput").value
                console.log(search)
                document.querySelectorAll("#nameElem").forEach((ne)=>
                    {
                        if (ne.textContent.toLowerCase().startsWith(search.toLowerCase()))
                            {
                                
                                ne.parentElement.style.display = "block"
                            }
                        
                        

                        else
                            {
                                ne.parentElement.style.display = "none";
                            }
                    })
                document.querySelectorAll("#name").forEach((nr)=>
                    {
                        if (nr.textContent.toLowerCase().startsWith(search.toLowerCase()))
                            {
                                
                                nr.parentElement.style.display = "table-row"
                            }
                        
                        

                        else
                            {
                                nr.parentElement.style.display = "none";
                            }
                    })

            })


        document.getElementById("tagInput").addEventListener("change",(e)=>
            {
                let filter =e.target.value;
                document.querySelectorAll("#category").forEach((er)=>
                    {
                        
                        if (er.textContent== filter|filter == "Select")
                            {
                                
                                er.parentElement.style.display = "table-row"
                            }
                        
                        

                        else
                            {
                                er.parentElement.style.display = "none";
                            }
                        

                    })
                document.querySelectorAll("#categoryElem").forEach((ed)=>
                    {
                        
                        if (ed.textContent== filter|filter == "Select")
                            {
                                
                                ed.parentElement.style.display = "block"
                            }
                        

                        else
                            {
                                ed.parentElement.style.display = "none";
                            }
                        

                    })
            })
        //popup
        document.getElementById("selectInput").addEventListener("change",(e)=>
            {
                
                if (e.target.value == "Select")
                    {
                        e.target.parentElement.querySelector("#submitButton").disabled= true
                        console.log(e.target.parentElement.querySelector("#submitButton").disabled)


                    }
                else
                    {
                        e.target.parentElement.querySelector("#submitButton").disabled = false
                        console.log(e.target.parentElement.querySelector("#submitButton").disabled)
                    }
            })


        //Amount Input
        document.getElementById("amountInput").addEventListener('input',(event)=>
            {
                console.log(event.target.value)
                if (event.target.value <0)
                    {
                        event.target.value = event.target.value * -1 ;
                    }

            })

        //price Input
        document.getElementById("priceInput").addEventListener('input',(event)=>
            {
                console.log(event.target.value)
                if (event.target.value <0)
                    {
                        event.target.value = event.target.value * -1 ;
                    }

            })
        
        //display Slider    
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

        


        
        document.getElementById("submitButton").addEventListener("click",(event)=>
            {
            
                const form = document.getElementById('popup');
                const formData = new FormData(form);
                console.log("Categories func",getSelectedCategories())
                formData.append("categories",[getSelectedCategories()])
                const formObject = {};

                // Convert FormData to a plain object for logging
                formData.forEach((value, key) => {
                    console.log(parseFloat(value))
                    if (isNaN(Number(value)))
                        {
                            formObject[key] =value;
                            if (key == "time")
                            {
                                let dato = new Date(value.toString())
                                console.log(dato.getTime())
                                formObject["time"] = dato.getTime()
                                formData.set("time",dato.getTime())
                            }
                        }
                    else{
                    formObject[key] = Number(value);
                    console.log(key," ",Number(value),)
                    if (key == "time")
                        {
                            let dato = new Date(value.toString())
                            console.log(dato.getTime())
                            formObject["time"] = dato.getTime()
                            formData.set("time",dato.getTime())
                        }}
                });
                
                let  response = fetch(`https://home-inventory-bml1.onrender.com/item`, 
                    {
                        method:"PUT",
                        headers: 
                            {
                                "Content-Type": "application/json"
                            },
                        body: JSON.stringify(formObject)
                    }).then((req,res)=>{console.log(req)})
        
                // Log to console
                console.log('Form Data:', JSON.stringify(formObject));
                location.reload()
        
            })
        document.getElementById("closeButton").addEventListener("click",(event)=>
        {
        event.target.parentElement.style.display = "none"
        document.querySelectorAll("#hidden-table").forEach(elem=>{elem.style.opacity = 1})
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
                         //if (screenWidth<1000){console.log(`${date.getMonth()}/${date.getDate()}/${date.getFullYear()}`)}
                        let Abv_date = `${date.getMonth()}/${date.getDate()}/${date.getFullYear()}`
                        let quantity = element.quantity
                        let unit_price = element.unit_price



                        console.log(`${date} -- ${element.date.$date.$numberLong}`)

                        //creating elements


                        //table display
                        let table_row = document.createElement("tr")

                        let th_name = document.createElement("th")
                        th_name.id = "name"
                        th_name.textContent =item_name

                        let th_category = document.createElement("th")
                        th_category.id = "category"
                        th_category.textContent = category

                        let th_date = document.createElement("th")
                        th_date.id = "dateElem"
                        th_date.textContent = date

                        let th_abv_date = document.createElement("th")
                        th_abv_date.id = "abv_date"
                        th_abv_date.textContent = Abv_date


                        let th_quantity = document.createElement("th")
                        th_quantity.id = "quantity"
                        th_quantity.textContent = `${quantity} ${method_measure}`

                        let th_unit_price = document.createElement("th")
                        th_unit_price.id = "unit_price"
                        th_unit_price.textContent = `$${unit_price.$numberDecimal}`


                        let th_hidden_id = document.createElement("th")
                        th_hidden_id.id="id"
                        th_hidden_id.textContent = id
                        th_hidden_id.style.display = "None"

                        

                        // let th_hidden_time=document.createElement("th")
                        // th_hidden_time.id = "time"
                        // th_hidden_time.textContent = date
                        let th_graph_button_row = document.createElement('th')
                        let th_graph_button =document.createElement("button")
                        th_graph_button.id = "graphbutton"
                        th_graph_button.textContent = "Graph"
                        th_graph_button.type = "button"
                        th_graph_button_row.appendChild(th_graph_button)
                        th_graph_button.addEventListener("click",(event)=>
                            {
                                let id = (event.target.parentElement.parentElement.querySelector("#id").textContent)
                                localStorage.setItem("item",id)
                                setCookie("item",id,1)
                            })



                        let th_edit_button_row = document.createElement("th")
                        let th_edit_button = document.createElement("button")
                        th_edit_button.id="editButton"
                        th_edit_button.textContent="Edit"
                        th_edit_button.innerHTML="Edit"
                        th_edit_button.type = "button"
                        th_edit_button.addEventListener("click",event=>
                            { 

                                let item = event.target.parentElement.parentElement
                                
                                console.log(item.querySelector("#unit_price").textContent.substring(1,(item.querySelector("#unit_price").length)))

                                document.getElementById("popup").style.display = "block"
                                document.querySelectorAll("#hidden-table").forEach(elem=>{elem.style.opacity = .5})
                                document.getElementById("idInput").value = item.querySelector('#id').textContent
                                document.getElementById("idLabel2").innerHTML = item.querySelector('#id').textContent
                                
                                //nameElem
                                document.getElementById("nameInput").value = item.querySelector('#name').textContent
                                
                                //categoryElem
                                document.querySelector(`[data-value=${item.querySelector("#category").textContent}]`).className="toggle-button selected"
                                

                                //method_measure
                                
                                document.getElementById("selectInput").value = item.querySelector("#quantity").innerText.split(" ")[1]

                                //quantityElem
                                document.getElementById("amountInput").value = item.querySelector("#quantity").textContent.split(" ")[0]
                                document.getElementById("oldAmount").value = item.querySelector("#quantity").textContent.split(" ")[0]


                                //unit_priceElem
                                document.getElementById("priceInput").value = item.querySelector("#unit_price").textContent.substring(1,(item.querySelector("#unit_price").length))

                                //dateElem
                                document.getElementById("timeInput").value = item.querySelector("#date").textContent
                                document.querySelectorAll("#editButton").forEach(elem=>{elem.disabled =true})
                                console.log(`X${scrollX}  Y${(scrollY + Number(50)) }`)

                                document.getElementById("popup").style.top = `${window.scrollY+100}px`;
                                document.getElementById("popup").style.left = `10%`;
                            
                            
                            })
                    
                        th_edit_button_row.appendChild(th_edit_button)
    
                        let th_delete_button_row = document.createElement("th")
                        let th_delete_button = document.createElement("button")
                        th_delete_button.id="deleteButton"
                        th_delete_button.textContent="Delete"
                        th_delete_button.type = "button"
                        th_delete_button_row.appendChild(th_delete_button) 

                        

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
                        section1Div.id ="section1Div"
                        section2Div.id = "section2Div"
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


                        let Abv_dateElem = document.createElement("p")
                        Abv_dateElem.textContent = Abv_date
                        Abv_dateElem.id = "Abv_dateElem"

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
                        edit_button.innerHTML="Edit"
                        edit_button.type = "button"
                        
                        
                        
                        let delete_button = document.createElement("button")
                        delete_button.id = "deleteButton"
                        delete_button.innerHTML = "Delete"
                        

                        edit_button.addEventListener("click",(event)=>
                            {
                                //console.log()
                                
                                
                                document.getElementById("popup").style.display = "block"
                                document.querySelectorAll("#items-wrapper").forEach(elem=>{elem.style.opacity = .5})
                                document.getElementById("idInput").value = event.target.parentElement.parentElement.parentElement.querySelector('.id').textContent
                                document.getElementById("idLabel2").innerHTML = event.target.parentElement.parentElement.parentElement.querySelector('.id').textContent
                                
                                //nameElem
                                document.getElementById("nameInput").value = event.target.parentElement.parentElement.parentElement.querySelector('#nameElem').textContent
                                
                                //categoryElem
                                event.target.parentElement.parentElement.parentElement.querySelector("#categoryElem").textContent.split(",").map((itemo)=>{console.log(itemo);document.querySelector(`[data-value=${itemo}]`).className="toggle-button selected"})
                                console.log( event.target.parentElement.parentElement.parentElement.querySelector("#categoryElem").textContent)

                                //method_measure
                                document.getElementById("selectInput").value = event.target.parentElement.parentElement.parentElement.querySelector('#method_measure').textContent

                                //quantityElem
                                document.getElementById("amountInput").value = event.target.parentElement.parentElement.parentElement.querySelector("#quantityElem").textContent
                                document.getElementById("oldAmount").value = event.target.parentElement.parentElement.parentElement.querySelector("#quantityElem").textContent
                                document.getElementById("timeInput").value = "w"
                                console.log(event.target.parentElement.parentElement.parentElement.querySelector("#dateElem").textContent)
                                

                                //unit_priceElem
                                document.getElementById("priceInput").value = event.target.parentElement.parentElement.parentElement.querySelector("#unit_priceElem").textContent.substring(1,event.target.parentElement.parentElement.parentElement.querySelector("#unit_priceElem").textContent.length)

                                //dateElem
                                document.getElementById("timeInput").value = event.target.parentElement.parentElement.parentElement.querySelector("#dateElem").textContent
                                document.querySelectorAll("#editButton").forEach(elem=>{elem.disabled =true})
                                console.log(`X${scrollX}  Y${scrollY +100}`)
                                document.getElementById("popup").style.top = `${window.scrollY+100}px`;
                                document.getElementById("popup").style.left = `10%`;

                            })


                        console.log(element)

                        //putting together the table

                        table_row.appendChild(th_name)
                        table_row.appendChild(th_category)
                        table_row.appendChild(th_date)
                        table_row.appendChild(th_abv_date)
                        table_row.appendChild(th_quantity)
                        table_row.appendChild(th_unit_price)
                        table_row.appendChild(th_hidden_id)                        
                        table_row.appendChild(th_graph_button_row)
                        table_row.appendChild(th_edit_button_row)
                        table_row.appendChild(th_delete_button_row)
                        

                        document.getElementById("inventory-table").appendChild(table_row)


                        //putting together the box display
                        box.appendChild(iNameElem)
                        box.appendChild(idElem)
                        box.appendChild(categoryElem)
                        box.appendChild(measureDiv)
                        box.appendChild(unit_priceElem)
                        box.appendChild(dateElem)
                        box.appendChild(Abv_dateElem)
                        buttonSection1.appendChild(edit_button)
                        buttonSection2.appendChild(delete_button)
                        box.appendChild(button_div)
                   
                        document.getElementById("items-box").appendChild(box)
                        
                        
                       //document.querySelectorAll("#items-wrapper").forEach((elem, index)=>{dragElement(elem);elem.style.top =`${index*600+100}px`});
                       
                       Object.keys(element).forEach(key => {
                        console.log(`${key}: ${isObject(element[key])}`);
                    })
                    }
                );document.querySelectorAll("#deleteButton").forEach((x)=>
                    {
                        x.addEventListener("click",(e)=>
                            {
                                console.log("elem",e.target.parentElement.parentElement);
                                if (e.target.parentElement.parentElement.tagName =="TR")
                                    {
                                        
                                        let x = e.target.parentElement.parentElement.querySelector("#id").textContent
                                        console.log(x)
                                        fetch("https://home-inventory-bml1.onrender.com/item",{method:"Delete",headers: 
                            {
                                "Content-Type": "application/json"
                            },body:JSON.stringify({"id":x})}).then(()=>
                                    {
                                        location.reload()
                                    })
                                }
                                else
                                    {
                                        
                                        let x = e.target.parentElement.parentElement.parentElement.querySelector("#id").textContent
                                        console.log(x)
                                        fetch("https://home-inventory-bml1.onrender.com/item",{method:"Delete",headers: 
                            {
                                "Content-Type": "application/json"
                            },body:JSON.stringify({"id":x})}).then(()=>
                                    {
                                        location.reload()
                                    })
                                    
                                }
                                    
                            })
                    })
            }
        )
        
    }) 



function edit_form(element)
    {
        console.log(element.target.parentElement.parentElement)



    }



function toggleSelection(element) 
        {
            element.classList.toggle('selected');
        }

function getSelectedCategories() 
    {
        const buttons = document.querySelectorAll('.toggle-button.selected');
        const selectedOptions = Array.from(buttons).map(button => button.getAttribute('data-value'));
        console.log(typeof selectedOptions)
        return selectedOptions
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
    e = e || window.e;
    e.preventDefault();
    // get the mouse cursor position at startup:
    pos3 = e.clientX;
    pos4 = e.clientY;
    document.onmouseup = closeDragElement;
    // call a function whenever the cursor moves:
    document.onmousemove = elementDrag;
  }

  function elementDrag(e) {
    e = e || window.e;
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
    document.onmouseup = null;
    document.onmousemove = null;
  }
}


