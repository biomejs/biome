// TanStack: Properly imported from @tanstack/react-query
import { useQuery } from '@tanstack/react-query';

function ValidComponent1({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should report missing userId
  });
}

// TanStack: Namespace import
import * as Query from '@tanstack/react-query';

function ValidComponent2({ userId }) {
  const query = Query.useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should report missing userId
  });
}

// TanStack: Experimental package
import { useQuery } from '@tanstack/react-query/experimental';

function ValidComponent3({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should report missing userId
  });
}

// TanStack: Legacy package name
import { useQuery } from 'react-query';

function ValidComponent4({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should report missing userId
  });
}

// Non‑TanStack: Not imported from TanStack Query package
import { useQuery } from 'some-other-library';

function InvalidComponent1({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should NOT report - not a TanStack Query hook
  });
}

// Global: No import at all (global function)
function InvalidComponent2({ userId }) {
  const query = useQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should still report as we're permissive for globals
  });
}

// Non‑TanStack: Different hook name should not trigger
import { useMutation } from 'some-other-library';

function InvalidComponent3({ userId }) {
  const query = useMutation({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should NOT report - not useQuery/useInfiniteQuery
  });
}

// TanStack: Alias import
import { useQuery as useReactQuery } from '@tanstack/react-query';

function AliasComponent({ userId }) {
  const query = useReactQuery({
    queryKey: ['users'],
    queryFn: () => fetchUser(userId) // Should report missing userId
  });
}