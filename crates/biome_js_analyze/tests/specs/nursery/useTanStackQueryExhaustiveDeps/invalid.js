// Missing userId in queryKey
function Component1({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId)
  });
}

// Missing multiple dependencies
function Component2({ userId, status }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUsersByStatus(userId, status)
  });
}

// Missing dependency in template literal
function Component3({ propertyId }) {
  const query = useQuery({
    queryKey: ['config'],
    queryFn: () => fetch(`/api/config/${propertyId}`).then(res => res.json())
  });
}

// Missing function parameter dependency
function usePublicConfig(propertyId) {
  return useQuery({
    queryKey: ['config'],
    queryFn: () => {
      return fetch(`/api/config/${propertyId}`)
        .then((res) => res.data);
    }
  });
}