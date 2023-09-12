<>
    <div dangerouslySetInnerHTML={{ __html: 'HTML' }}>children</div>
    <div dangerouslySetInnerHTML={{ __html: 'HTML' }} children={'children'} />
    <div dangerouslySetInnerHTML={{ __html: 'HTML' }} children={['children']} />
    <Invalid dangerouslySetInnerHTML={{ __html: 'HTML' }}>children</Invalid>
    <Invalid dangerouslySetInnerHTML={{ __html: 'HTML' }} children={'children'} />
</>