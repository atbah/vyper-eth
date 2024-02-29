import { useContext } from 'react';
import './App.css';
import { CreateConfigPage } from './pages/create-config';
import { PageContext, Pages } from './layout';
import { HomePage } from './pages/home';
import { GenerateKeystorePage } from './pages/generate-keystore';
import { DeployContractPage } from './pages/deploy-contract'

function App() {
  const { currentPage } = useContext(PageContext);

  if (currentPage === Pages.Home) {
    return <HomePage />;
  }

  if (currentPage === Pages.CreateConfig) {
    return <CreateConfigPage />;
  }

  if (currentPage === Pages.GenerateKeystore) {
    return <GenerateKeystorePage />;
  }

  if (currentPage === Pages.DeployContract) {
    return <DeployContractPage />;
  }
}

export default App;
