import {
  createContext,
  type ReactNode,
  useCallback,
  useContext,
  useState,
} from "react";

const SESSION_EXPIRES_AT_KEY = "session_expires_at";

const getLocalSessionExpires = (): Date | undefined => {
  const strSessionExpires = localStorage.getItem(SESSION_EXPIRES_AT_KEY) ?? "";
  const numSessionExpires = Number.parseInt(strSessionExpires, 10);
  const dateSessionExpires = new Date(numSessionExpires);

  return Number.isNaN(dateSessionExpires.getTime())
    ? undefined
    : dateSessionExpires;
};

const setLocalSessionExpires = (date: Date): void => {
  localStorage.setItem(SESSION_EXPIRES_AT_KEY, date.getTime().toString());
};

interface AuthContextType {
  isAuthenticated: boolean;
  setSessionExpires: (d: Date) => void;
}

const isAuthenticated = () => {
  const sessionExpires = getLocalSessionExpires();
  return !!sessionExpires && sessionExpires.getTime() > Date.now();
};

const AuthContext = createContext<AuthContextType>({
  isAuthenticated: isAuthenticated(),
  setSessionExpires: setLocalSessionExpires,
});

const AuthContextProvider = ({ children }: { children: ReactNode }) => {
  // TODO: make this expire when session expires
  const [sessionExpires, _setSessionExpires] = useState(
    getLocalSessionExpires(),
  );

  const setSessionExpires = useCallback((next: Date) => {
    setLocalSessionExpires(next);
    _setSessionExpires(next);
  }, []);

  const value = {
    isAuthenticated: !!sessionExpires && sessionExpires.getTime() > Date.now(),
    setSessionExpires,
  };

  return <AuthContext value={value}>{children}</AuthContext>;
};

const useisAuthenticated = () => useContext(AuthContext).isAuthenticated;
const useSetSessionExpires = () => useContext(AuthContext).setSessionExpires;

export {
  AuthContextProvider,
  useisAuthenticated,
  useSetSessionExpires,
  isAuthenticated,
};
