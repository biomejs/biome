/* should not generate diagnostics */

// TanStack Router pattern - component referenced in exported object
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/')({
  component: HomeComponent,
})

function HomeComponent() {
  return <div>Home</div>
}
