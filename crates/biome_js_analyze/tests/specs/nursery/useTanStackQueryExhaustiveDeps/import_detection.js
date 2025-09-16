// Valid: Properly imported from @tanstack/react-query
import { useQuery } from '@tanstack/react-query';

function ValidComponent1({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should report missing userId
  });
}

// Valid: Namespace import
import * as Query from '@tanstack/react-query';

function ValidComponent2({ userId }) {
  const query = Query.useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should report missing userId
  });
}

// Valid: Experimental package
import { useQuery } from '@tanstack/react-query/experimental';

function ValidComponent3({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should report missing userId
  });
}

// Valid: Legacy package name
import { useQuery } from 'react-query';

function ValidComponent4({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should report missing userId
  });
}

// Invalid: Not imported from TanStack Query package
import { useQuery } from 'some-other-library';

function InvalidComponent1({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should NOT report - not a TanStack Query hook
  });
}

// Invalid: No import at all (global function)
function InvalidComponent2({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should still report as we're permissive for globals
  });
}

// Valid: Different hook name from wrong package should not trigger
import { useMutation } from 'some-other-library';

function InvalidComponent3({ userId }) {
  const query = useMutation({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should NOT report - not useQuery/useInfiniteQuery
  });
}