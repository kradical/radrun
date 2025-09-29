import { createRootRoute, Link, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { useisAuthenticated } from "src/auth/AuthContext";

const RootLayout = () => {
  const isAuthenticated = useisAuthenticated();

  return (
    <>
      <div className="p-2 flex gap-2">
        {isAuthenticated && (
          <>
            <Link to="/" className="[&.active]:font-bold">
              Home
            </Link>{" "}
            <Link to="/about" className="[&.active]:font-bold">
              About
            </Link>
          </>
        )}

        {!isAuthenticated && (
          <>
            <Link to="/login" className="[&.active]:font-bold">
              Login
            </Link>{" "}
            <Link to="/sign-up" className="[&.active]:font-bold">
              Sign Up
            </Link>
          </>
        )}
      </div>
      <hr />
      <Outlet />
      <TanStackRouterDevtools />
    </>
  );
};

export const Route = createRootRoute({ component: RootLayout });
