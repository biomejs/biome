const heading: any = document.getElementsByTagName('h3')[0]
heading.style.textAlign = 'center';
heading.style.marginTop = '150px';
heading.style.fontSize = '50px';
const container: any = document.getElementsByClassName('contributors');
const list: any = container[0].children;

// Hide all items after the first 50
for(let i = 0; i < 35; i++) {
    list[i].style.width = 'fit-content';
    list[i].style.height = 'fit-content';
    list[i].firstChild.style.width = 'max-content';
    list[i].firstChild.style.display = 'flex';
    list[i].firstChild.lastChild.style.display = 'none';
    list[i].firstChild.style.alignItems = 'center';
    list[i].firstChild.style.flexDirection = 'column';
    let image = list[i].firstChild.firstChild;
    image.style.height = '70px';
    image.style.width = '70px';
    image.style.borderRadius = '999px';
}

list[0].style.visibility = 'hidden';
list[10].style.visibility = 'hidden';
list[20].style.visibility = 'hidden';
list[21].style.visibility = 'hidden';
list[25].style.visibility = 'hidden';

for (let i = 35; i < list.length; i++) {
  list[i].style.display = 'none';
}