const applyStyles = (element: any, styles: any) => {
  Object.assign(element.style, styles);
};

const hideAvatars = (element: any) => {
  element.style.display = 'none';
};

const hideElement = (element: any) => {
  element.style.visibility = 'hidden';
};

const heading: any = document.getElementsByTagName('h3')[0]
applyStyles(heading, {
  textAlign: 'center',
  marginTop: '150px',
  fontSize: '50px',
});

const container: any = document.getElementsByClassName('contributors');
const list: any = container[0].children;

for(let i = 0; i < 35; i++) {
  const listItem = list[i];
  const listItemContent = listItem.firstChild;
  const listItemContentLastChild = listItemContent.lastChild;

  applyStyles(listItem, {
    width: 'fit-content',
    height: 'fit-content',
    border: 'none',
    margin: '7px',
  });

  applyStyles(listItemContent, {
    width: 'max-content',
    display: 'flex',
    alignItems: 'center',
    flexDirection: 'column',
  });

  applyStyles(listItemContentLastChild, {
    display: 'none',
  });

  const image = listItemContent.firstChild;
  applyStyles(image, {
    height: '70px',
    width: '70px',
    borderRadius: '999px',
  });
}

// Hide specific list items
[0, 10, 20, 21, 25].forEach((index) => hideElement(list[index]));

// Hide items beyond index 34
for (let i = 35; i < list.length; i++) {
  hideAvatars(list[i]);
}
